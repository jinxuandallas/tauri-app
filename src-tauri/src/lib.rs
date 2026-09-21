mod schema;
use libsql::Database;
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
                let url = r"libsql://test-jinxuandallas.aws-ap-northeast-1.turso.io".to_string();
                let auth_token = r"eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCJ9.eyJhIjoicnciLCJpYXQiOjE3ODg3MDAzNTksImlkIjoiMDFhMDZhNDUtYjUwMS03OGMyLTlmZGEtYzE5YTRlNWM0Njg0Iiwia2lkIjoiZnB2eVZsSWFZQmp1NjgtYS1TX0R6Y2ttbTlCbWtweENWTjFiUmtyNFctVSIsInJpZCI6IjZlZmI4YmJhLTdkODgtNGM1OS1iMDJkLWQxYTU5YjZhNmE2OSJ9.BDDyi_-WtXAf1eaJYHboK0sRK-ePRsoNN8HHprYdvUpwEZ8AEBTcRWL70oJgecWkyHW6zZ39lAnWdF3zjexpCg".to_string(); // 你的 token

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
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![greet, test_turso])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
