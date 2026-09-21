mod schema;
use libsql::Database;
use std::fs;
use std::{env, sync::Arc};
use tauri::{Manager, State};
use tauri_plugin_fs::FsExt;
use tokio::sync::Mutex;

struct DbState {
    db: Arc<Mutex<Database>>,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn test_turso(state: State<'_, DbState>) -> Result<String, String> {
    /*
    // 1. 获取资源目录（Android 上返回 asset://localhost/）
    let resource_dir = app_handle
        .path()
        .resource_dir()
        .map_err(|e| format!("无法获取资源目录: {}", e))?;

    // 2. 使用 tauri-plugin-fs 读取资源文件内容
    let cert_data = app_handle
        .fs()
        .read_to_string(resource_dir.join("cacert.pem"))
        .map_err(|e| format!("无法读取证书文件: {}", e))?;

    // 3. 获取应用内部可写目录
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("无法获取应用数据目录: {}", e))?;

    // 4. 将证书写入内部存储（rustls 需要真实文件系统路径）
    let cert_path = app_data_dir.join("cacert.pem");
    std::fs::write(&cert_path, cert_data.as_bytes())
        .map_err(|e| format!("无法写入证书文件: {}", e))?;

    // 5. 设置环境变量
    std::env::set_var("SSL_CERT_FILE", &cert_path);
    println!("SSL_CERT_FILE 已设置为: {:?}", cert_path);

    // --- 第二步：连接数据库 ---

    // 你的数据库连接代码保持不变
    let url = r"libsql://test-jinxuandallas.aws-ap-northeast-1.turso.io".to_string();
    let auth_token = r"eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCJ9.eyJhIjoicnciLCJpYXQiOjE3ODg3MDAzNTksImlkIjoiMDFhMDZhNDUtYjUwMS03OGMyLTlmZGEtYzE5YTRlNWM0Njg0Iiwia2lkIjoiZnB2eVZsSWFZQmp1NjgtYS1TX0R6Y2ttbTlCbWtweENWTjFiUmtyNFctVSIsInJpZCI6IjZlZmI4YmJhLTdkODgtNGM1OS1iMDJkLWQxYTU5YjZhNmE2OSJ9.BDDyi_-WtXAf1eaJYHboK0sRK-ePRsoNN8HHprYdvUpwEZ8AEBTcRWL70oJgecWkyHW6zZ39lAnWdF3zjexpCg".to_string(); // 你的 token

    let db = libsql::Builder::new_remote(url, auth_token)
        .build()
        .await
        .map_err(|e| format!("数据库连接失败: {}", e))?;

    println!("成功连接到 Turso 数据库！");
    */

    let db = state.db.lock().await;
    let conn = db.connect().unwrap();

    let mut result = String::new();
    let mut rows = conn.query("select * from test1", ()).await.unwrap();
    while let Some(row) = rows.next().await.unwrap() {
        let name: String = row.get(1).unwrap();
        let id = row.get::<i32>(0).unwrap();
        println!("name: {}, id: {}", name, id);
        result += &format!("name: {}, id: {}<br/>", name, id);
    }

    Ok(result)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let app_handle = app.handle();

            // 1. 获取资源目录（Android 上返回 asset://localhost/）
            let resource_dir = app_handle
                .path()
                .resource_dir()
                .map_err(|e| format!("无法获取资源目录: {}", e))?;

            // 2. 使用 tauri-plugin-fs 读取资源文件内容
            let cert_data = app_handle
                .fs()
                .read_to_string(resource_dir.join("cacert.pem"))
                .map_err(|e| format!("无法读取证书文件: {}", e))?;

            // 3. 获取应用内部可写目录
            let app_data_dir = app_handle
                .path()
                .app_data_dir()
                .map_err(|e| format!("无法获取应用数据目录: {}", e))?;

            // 4. 将证书写入内部存储（rustls 需要真实文件系统路径）
            let cert_path = app_data_dir.join("cacert.pem");
            std::fs::write(&cert_path, cert_data.as_bytes())
                .map_err(|e| format!("无法写入证书文件: {}", e))?;

            // 5. 设置环境变量
            std::env::set_var("SSL_CERT_FILE", &cert_path);
            println!("SSL_CERT_FILE 已设置为: {:?}", cert_path);

            let db_state = tauri::async_runtime::block_on(async {
                // 1. 获取应用专属的外部存储目录（在 Android 上通常是 /storage/emulated/0/Android/data/<package>/files）
                let app_dir = app.path().app_config_dir().expect("无法获取外部存储目录");

                let config_file_path = app_dir.join("config.toml");

                // 2. 如果外部配置文件不存在，则从资源中复制

                let content;
                if !config_file_path.exists() {
                    // 读取打包在资源中的配置文件
                    // resolve_resource 返回的是 asset:// URI，需要用 fs 插件读取
                    let resource_path = app
                        .path()
                        .resolve(
                            "resources/config.toml",
                            tauri::path::BaseDirectory::Resource,
                        )
                        .expect("无法解析资源路径");

                    // 使用 fs 插件读取资源文件内容
                    content = app
                        .fs()
                        .read_to_string(&resource_path)
                        .expect("无法读取资源文件");

                    // 确保目录存在
                    fs::create_dir_all(&app_dir).expect("无法创建应用目录");

                    // 写入到外部存储
                    fs::write(&config_file_path, &content).expect("无法写入外部配置文件");

                    println!("配置文件已从资源复制到: {:?}", config_file_path);
                } else {
                    content = app
                        .fs()
                        .read_to_string(&config_file_path)
                        .expect("无法读取资源文件");
                }

                let app_config: toml::Value = toml::from_str(&content).unwrap();
                let url = app_config["TURSO_DATABASE_URL"]
                    .as_str()
                    .unwrap()
                    .to_string();
                let auth_token = app_config["TURSO_AUTH_TOKEN"].as_str().unwrap().to_string();
                // let url = r"libsql://test-jinxuandallas.aws-ap-northeast-1.turso.io".to_string();
                // let auth_token = r"eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCJ9.eyJhIjoicnciLCJpYXQiOjE3ODg3MDAzNTksImlkIjoiMDFhMDZhNDUtYjUwMS03OGMyLTlmZGEtYzE5YTRlNWM0Njg0Iiwia2lkIjoiZnB2eVZsSWFZQmp1NjgtYS1TX0R6Y2ttbTlCbWtweENWTjFiUmtyNFctVSIsInJpZCI6IjZlZmI4YmJhLTdkODgtNGM1OS1iMDJkLWQxYTU5YjZhNmE2OSJ9.BDDyi_-WtXAf1eaJYHboK0sRK-ePRsoNN8HHprYdvUpwEZ8AEBTcRWL70oJgecWkyHW6zZ39lAnWdF3zjexpCg".to_string(); // 你的 token

                println!("u:{},t:{}", url, &auth_token[1..21]);
                libsql::Builder::new_remote(url, auth_token)
                    .build()
                    .await
                    .map_err(|e| format!("数据库连接失败: {}", e))
                    .unwrap()
            });
            println!("Connected to Turso database: ");
            app.manage(DbState {
                db: Arc::new(Mutex::new(db_state)),
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, test_turso])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
