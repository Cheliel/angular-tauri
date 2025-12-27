use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{command, AppHandle, Manager};

#[derive(Debug, Serialize, Deserialize)]
pub struct NumbersData {
    numbers: Vec<f64>,
    created_at: String,
    updated_at: String,
}

impl NumbersData {
    fn new(numbers: Vec<f64>) -> Self {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
        Self {
            numbers,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    fn update_numbers(&mut self, numbers: Vec<f64>) {
        self.numbers = numbers;
        self.updated_at = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
    }
}

fn get_data_file_path(app: &AppHandle) -> Result<PathBuf, String> {
    let app_dir = app.path_resolver()
        .app_data_dir()
        .ok_or("Impossible de résoudre le répertoire de données de l'app")?;
    
    // Créer le répertoire s'il n'existe pas
    fs::create_dir_all(&app_dir)
        .map_err(|e| format!("Erreur création répertoire: {}", e))?;
    
    Ok(app_dir.join("numbers_data.json"))
}

// ===== COMMANDES POUR LE SERVICE ANGULAR =====

#[command]
pub async fn load_numbers(app: AppHandle) -> Result<NumbersData, String> {
    match load_numbers_data(&app).await {
        Ok(data) => Ok(data),
        Err(_) => {
            // Si aucun fichier n'existe, retourner données vides
            Ok(NumbersData::new(vec![]))
        }
    }
}

#[command]
pub async fn save_numbers(app: AppHandle, numbers: Vec<f64>) -> Result<bool, String> {
    let file_path = get_data_file_path(&app)?;
    
    // Charger les données existantes ou créer nouvelles
    let mut data = match load_numbers_data(&app).await {
        Ok(existing_data) => {
            let mut data = existing_data;
            data.update_numbers(numbers);
            data
        }
        Err(_) => NumbersData::new(numbers),
    };
    
    // Sérialiser en JSON avec indentation
    let json_content = serde_json::to_string_pretty(&data)
        .map_err(|e| format!("Erreur sérialisation JSON: {}", e))?;
    
    // Écrire dans le fichier
    fs::write(&file_path, json_content)
        .map_err(|e| format!("Erreur écriture fichier: {}", e))?;
    
    Ok(true)
}

#[command]
pub async fn clear_numbers(app: AppHandle) -> Result<bool, String> {
    save_numbers(app, vec![]).await
}

#[command]
pub async fn get_numbers_stats(app: AppHandle) -> Result<serde_json::Value, String> {
    match load_numbers_data(&app).await {
        Ok(data) => {
            if data.numbers.is_empty() {
                return Ok(serde_json::json!({
                    "count": 0,
                    "sum": 0,
                    "average": 0,
                    "min": 0,
                    "max": 0
                }));
            }
            
            let count = data.numbers.len();
            let sum: f64 = data.numbers.iter().sum();
            let average = sum / count as f64;
            let min = data.numbers.iter().fold(f64::INFINITY, |a, &b| a.min(b));
            let max = data.numbers.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
            
            Ok(serde_json::json!({
                "count": count,
                "sum": sum,
                "average": average,
                "min": min,
                "max": max,
                "created_at": data.created_at,
                "updated_at": data.updated_at
            }))
        }
        Err(_) => Ok(serde_json::json!({
            "count": 0,
            "sum": 0,
            "average": 0,
            "min": 0,
            "max": 0
        }))
    }
}

// ===== FONCTIONS UTILITAIRES =====

async fn load_numbers_data(app: &AppHandle) -> Result<NumbersData, String> {
    let file_path = get_data_file_path(app)?;
    
    if !file_path.exists() {
        return Err("Fichier de données non trouvé".to_string());
    }
    
    let json_content = fs::read_to_string(&file_path)
        .map_err(|e| format!("Erreur lecture fichier: {}", e))?;
    
    let data: NumbersData = serde_json::from_str(&json_content)
        .map_err(|e| format!("Erreur désérialisation JSON: {}", e))?;
    
    Ok(data)
}

#[command]
pub async fn get_numbers_info(app: AppHandle) -> Result<String, String> {
    match load_numbers_data(&app).await {
        Ok(data) => {
            let info = format!(
                "Fichier: {} nombres\nCréé: {}\nModifié: {}",
                data.numbers.len(),
                data.created_at,
                data.updated_at
            );
            Ok(info)
        }
        Err(_) => Ok("Aucune donnée sauvegardée".to_string()),
    }
}

#[command]
pub async fn clear_all_data(app: AppHandle) -> Result<String, String> {
    let file_path = get_data_file_path(&app)?;
    
    if file_path.exists() {
        fs::remove_file(&file_path)
            .map_err(|e| format!("Erreur suppression fichier: {}", e))?;
        Ok("Toutes les données ont été supprimées".to_string())
    } else {
        Ok("Aucune donnée à supprimer".to_string())
    }
}
