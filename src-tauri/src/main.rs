use std::fs;

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn scan(dir: &str) -> String {
    // let current_dir = std::env::current_dir().expect("todo");
    let files = fs::read_dir(dir).expect("todo");
    files.map(|file| {
        let file_entry = file.expect("todo");
        let file_path = file_entry.path();
        if let Some(file_name) = file_path.file_name() {
            file_name.to_string_lossy().to_string()
        } else { String::new() }
    }).collect::<Vec<String>>().join("\n")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![scan])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
