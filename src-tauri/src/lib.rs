// 引入新的 db 模块
mod db;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // 在应用启动时，执行数据库初始化
        .setup(|app| {
            db::init_database(&app.handle());
            Ok(())
        })
        // 注册所有后端命令
        .invoke_handler(tauri::generate_handler![
            greet,
            db::add_project,
            db::edit_project,
            db::delete_project,
            db::get_project_page
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
