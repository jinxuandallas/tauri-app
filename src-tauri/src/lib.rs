mod schema;
use std::env;
use tauri::Manager;
use tauri_plugin_fs::FsExt;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn test_turso(app_handle: tauri::AppHandle) -> Result<String, String> {
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
    let auth_token = r"eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCJ9...".to_string(); // 你的 token

    let _db = libsql::Builder::new_remote(url, auth_token)
        .build()
        .await
        .map_err(|e| format!("数据库连接失败: {}", e))?;

    println!("成功连接到 Turso 数据库！");
    Ok("test".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        /*
        .setup(|app| {
            dotenv().ok();
            let url = env::var("TURSO_DATABASE_URL").expect("TURSO_DATABASE_URL not set");
            let auth_token = env::var("TURSO_AUTH_TOKEN").expect("TURSO_AUTH_TOKEN not set");
            let db_state = tauri::async_runtime::block_on(async {
                libsql::Builder::new_remote(url, auth_token)
                    .build()
                    .await
                    .expect("db connection failed");
            });
            println!("Connected to Turso database: ");
            app.manage(db_state);
            Ok(())
        }) */
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![greet, test_turso])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
