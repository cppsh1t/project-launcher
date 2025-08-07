use once_cell::sync::Lazy;
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::AppHandle;
use tauri::Manager; 

// --- 全局数据库连接 ---
// 使用 Lazy 和 Mutex 来创建一个线程安全的、全局唯一的数据库连接实例
pub static DB: Lazy<Mutex<Connection>> = Lazy::new(|| {
    let conn = Connection::open_in_memory().expect("Failed to open in-memory database for init.");
    Mutex::new(conn)
});

// --- 数据结构定义 (与 TypeScript 对应) ---
// serde(rename_all = "camelCase") 是关键，它能自动转换 Rust 的 snake_case 和 TS 的 camelCase
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    id: Option<i64>, // 数据库中的 id 是 i64
    name: String,
    project_name: String,
    location: String,
    tags: Vec<String>,
    desc: String,
    launcher: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedProjects {
    list: Vec<Project>,
    total: i64,
}

// --- 数据库初始化 ---
pub fn init_database(app_handle: &AppHandle) {
    let app_dir = app_handle.path()
        .app_data_dir()
        .expect("The app data directory should exist.");
    std::fs::create_dir_all(&app_dir).expect("Failed to create app data dir");
    let db_path = app_dir.join("projects.db");

    let mut db_conn = DB.lock().unwrap();
    *db_conn = Connection::open(db_path).expect("Failed to open database file.");

    db_conn.execute(
        "CREATE TABLE IF NOT EXISTS projects (
            id            INTEGER PRIMARY KEY AUTOINCREMENT,
            name          TEXT NOT NULL,
            project_name  TEXT NOT NULL UNIQUE,
            location      TEXT NOT NULL,
            tags          TEXT NOT NULL, -- Stored as JSON string
            desc          TEXT NOT NULL,
            launcher      TEXT
        )",
        [],
    ).expect("Failed to create table 'projects'");
}

// --- Tauri Commands ---

#[tauri::command]
pub fn add_project(form: Project) -> Result<(), String> {
    let conn = DB.lock().unwrap();
    // 将 Vec<String> 序列化为 JSON 字符串以便存储
    let tags_json = serde_json::to_string(&form.tags).map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO projects (name, project_name, location, tags, desc, launcher) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![form.name, form.project_name, form.location, tags_json, form.desc, form.launcher],
    ).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn edit_project(form: Project) -> Result<(), String> {
    let conn = DB.lock().unwrap();
    let tags_json = serde_json::to_string(&form.tags).map_err(|e| e.to_string())?;
    
    conn.execute(
        "UPDATE projects SET name = ?1, project_name = ?2, location = ?3, tags = ?4, desc = ?5, launcher = ?6 WHERE id = ?7",
        params![form.name, form.project_name, form.location, tags_json, form.desc, form.launcher, form.id],
    ).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn delete_project(id: i64) -> Result<(), String> {
    let conn = DB.lock().unwrap();
    conn.execute("DELETE FROM projects WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_project_page(page: usize, size: usize, name: Option<String>, tags: Option<Vec<String>>) -> Result<PaginatedProjects, String> {
    let conn = DB.lock().unwrap();
    let offset = (page.saturating_sub(1)) * size;
    let mut where_clauses = Vec::new();
    // params_list is correctly built as Vec<String>
    let mut params_list: Vec<String> = Vec::new();
    if let Some(n) = name {
        if !n.is_empty() {
            where_clauses.push("(name LIKE ? OR project_name LIKE ?)".to_string());
            let pattern = format!("%{}%", n);
            params_list.push(pattern.clone());
            params_list.push(pattern);
        }
    }
    if let Some(t) = tags {
        if !t.is_empty() {
            for tag in t {
                where_clauses.push("tags LIKE ?".to_string());
                params_list.push(format!("%\"{}\"%", tag));
            }
        }
    }
    
    let where_sql = if where_clauses.is_empty() { "".to_string() } else { format!("WHERE {}", where_clauses.join(" AND ")) };
    // 查询总数
    let total_query = format!("SELECT COUNT(*) FROM projects {}", where_sql);
    // --- FIX 1: Use params_from_iter ---
    let total: i64 = conn.query_row(
        &total_query,
        rusqlite::params_from_iter(params_list.iter()), // Correct way to pass dynamic params
        |row| row.get(0)
    ).unwrap_or(0);
    
    // 查询分页数据
    let query = format!("SELECT id, name, project_name, location, tags, desc, launcher FROM projects {} ORDER BY id DESC LIMIT {} OFFSET {}", where_sql, size, offset);
    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    
    // --- FIX 2: Use params_from_iter again ---
    let project_iter = stmt.query_map(
        rusqlite::params_from_iter(params_list.iter()), // Correct way to pass dynamic params
        |row| {
            let tags_json: String = row.get(4)?;
            let tags: Vec<String> = serde_json::from_str(&tags_json).unwrap_or_else(|_| vec![]);
            Ok(Project {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                project_name: row.get(2)?,
                location: row.get(3)?,
                tags,
                desc: row.get(5)?,
                launcher: row.get(6)?,
            })
    }).map_err(|e| e.to_string())?;
    let list: Vec<Project> = project_iter.collect::<Result<Vec<Project>, _>>().map_err(|e| e.to_string())?;
    Ok(PaginatedProjects { list, total })
}