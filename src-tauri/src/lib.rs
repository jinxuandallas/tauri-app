mod schema;
use dotenv::dotenv;
use std::env;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn test_turso() -> String {
    // dotenv().ok();
    // let url = env::var("TURSO_DATABASE_URL").expect("TURSO_DATABASE_URL not set");
    // let auth_token = env::var("TURSO_AUTH_TOKEN").expect("TURSO_AUTH_TOKEN not set");
    let url = r"libsql://test-jinxuandallas.aws-ap-northeast-1.turso.io".to_string();
    let auth_token = r"eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCJ9.eyJhIjoicnciLCJpYXQiOjE3ODg3MDAzNTksImlkIjoiMDFhMDZhNDUtYjUwMS03OGMyLTlmZGEtYzE5YTRlNWM0Njg0Iiwia2lkIjoiZnB2eVZsSWFZQmp1NjgtYS1TX0R6Y2ttbTlCbWtweENWTjFiUmtyNFctVSIsInJpZCI6IjZlZmI4YmJhLTdkODgtNGM1OS1iMDJkLWQxYTU5YjZhNmE2OSJ9.BDDyi_-WtXAf1eaJYHboK0sRK-ePRsoNN8HHprYdvUpwEZ8AEBTcRWL70oJgecWkyHW6zZ39lAnWdF3zjexpCg".to_string();
    let db_state = tauri::async_runtime::block_on(async {
        libsql::Builder::new_remote(url, auth_token)
            .build()
            .await
            .expect("db connection failed");
    });
    println!("Connected to Turso database: ");
    println!("xx");
    "test".to_string()
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
        .invoke_handler(tauri::generate_handler![greet, test_turso])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
