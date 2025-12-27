// Empêche l'ouverture de la console sur Windows en mode release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod table_storage;

use table_storage::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // Commandes pour le tableau de nombres (compatibles avec TauriService)
            load_numbers,
            save_numbers,
            clear_numbers,
            get_numbers_stats,
            get_numbers_info,
            clear_all_data,
            
            // Autres commandes existantes si vous en avez
            greet // par exemple
        ])
        .run(tauri::generate_context!())
        .expect("Erreur lors du lancement de l'application Tauri");
}

// Commande d'exemple existante
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Bonjour, {} ! Vous avez été salué depuis Rust!", name)
}