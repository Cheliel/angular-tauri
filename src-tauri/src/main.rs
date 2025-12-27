// Prevent console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod numbers;

use numbers::*;

#[tauri::command]
async fn hello_world_command(_app: tauri::AppHandle) -> Result<String, String> {
  println!("I was invoked from JS!");
  Ok("Hello world from Tauri!".into())
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            load_numbers,
            save_numbers,
            clear_numbers,
            get_numbers_stats,
            add_number,
            remove_number,
            export_numbers_csv
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
        }
        