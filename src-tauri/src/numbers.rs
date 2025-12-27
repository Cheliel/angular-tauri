use serde::{Deserialize, Serialize};
use tauri::{command, AppHandle};
use std::fs;
use std::path::PathBuf;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct NumberData {
    pub numbers: Vec<f64>,
    pub last_modified: DateTime<Utc>,
}

impl Default for NumberData {
    fn default() -> Self {
        Self {
            numbers: Vec::new(),
            last_modified: Utc::now(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NumberStats {
    pub count: usize,
    pub sum: f64,
    pub average: f64,
    pub min: f64,
    pub max: f64,
    pub median: f64,
}

// Obtenir le chemin du fichier de données
fn get_data_file_path(app: &AppHandle) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let app_dir = app.path_resolver()
        .app_data_dir()
        .ok_or("Impossible de résoudre le répertoire de données")?;
    
    std::fs::create_dir_all(&app_dir)?;
    Ok(app_dir.join("numbers.json"))
}

// Charger les données depuis le fichier
fn load_data_from_file(file_path: &PathBuf) -> Result<NumberData, Box<dyn std::error::Error>> {
    if !file_path.exists() {
        return Ok(NumberData::default());
    }
    
    let content = fs::read_to_string(file_path)?;
    let data: NumberData = serde_json::from_str(&content)?;
    Ok(data)
}

// Sauvegarder les données dans le fichier
fn save_data_to_file(file_path: &PathBuf, data: &NumberData) -> Result<(), Box<dyn std::error::Error>> {
    let content = serde_json::to_string_pretty(data)?;
    fs::write(file_path, content)?;
    Ok(())
}

// === COMMANDES TAURI ===

#[command]
pub async fn load_numbers(app: AppHandle) -> Result<NumberData, String> {
    let file_path = get_data_file_path(&app).map_err(|e| e.to_string())?;
    load_data_from_file(&file_path).map_err(|e| e.to_string())
}

#[command]
pub async fn save_numbers(app: AppHandle, numbers: Vec<f64>) -> Result<bool, String> {
    let file_path = get_data_file_path(&app).map_err(|e| e.to_string())?;
    
    let data = NumberData {
        numbers,
        last_modified: Utc::now(),
    };
    
    save_data_to_file(&file_path, &data).map_err(|e| e.to_string())?;
    Ok(true)
}

#[command]
pub async fn clear_numbers(app: AppHandle) -> Result<bool, String> {
    let file_path = get_data_file_path(&app).map_err(|e| e.to_string())?;
    
    let data = NumberData::default();
    save_data_to_file(&file_path, &data).map_err(|e| e.to_string())?;
    Ok(true)
}

#[command]
pub async fn get_numbers_stats(app: AppHandle) -> Result<NumberStats, String> {
    let file_path = get_data_file_path(&app).map_err(|e| e.to_string())?;
    let data = load_data_from_file(&file_path).map_err(|e| e.to_string())?;
    
    if data.numbers.is_empty() {
        return Ok(NumberStats {
            count: 0,
            sum: 0.0,
            average: 0.0,
            min: 0.0,
            max: 0.0,
            median: 0.0,
        });
    }
    
    let count = data.numbers.len();
    let sum: f64 = data.numbers.iter().sum();
    let average = sum / count as f64;
    let min = data.numbers.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max = data.numbers.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    
    // Calculer la médiane
    let mut sorted_numbers = data.numbers.clone();
    sorted_numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median = if count % 2 == 0 {
        (sorted_numbers[count / 2 - 1] + sorted_numbers[count / 2]) / 2.0
    } else {
        sorted_numbers[count / 2]
    };
    
    Ok(NumberStats {
        count,
        sum,
        average,
        min,
        max,
        median,
    })
}

#[command]
pub async fn add_number(app: AppHandle, number: f64) -> Result<bool, String> {
    let file_path = get_data_file_path(&app).map_err(|e| e.to_string())?;
    let mut data = load_data_from_file(&file_path).map_err(|e| e.to_string())?;
    
    // Vérifier si le nombre existe déjà
    if data.numbers.contains(&number) {
        return Err("Ce nombre existe déjà".to_string());
    }
    
    data.numbers.push(number);
    data.last_modified = Utc::now();
    
    save_data_to_file(&file_path, &data).map_err(|e| e.to_string())?;
    Ok(true)
}

#[command]
pub async fn remove_number(app: AppHandle, index: usize) -> Result<bool, String> {
    let file_path = get_data_file_path(&app).map_err(|e| e.to_string())?;
    let mut data = load_data_from_file(&file_path).map_err(|e| e.to_string())?;
    
    if index >= data.numbers.len() {
        return Err("Index invalide".to_string());
    }
    
    data.numbers.remove(index);
    data.last_modified = Utc::now();
    
    save_data_to_file(&file_path, &data).map_err(|e| e.to_string())?;
    Ok(true)
}

#[command]
pub async fn export_numbers_csv(app: AppHandle) -> Result<String, String> {
    let file_path = get_data_file_path(&app).map_err(|e| e.to_string())?;
    let data = load_data_from_file(&file_path).map_err(|e| e.to_string())?;
    
    let mut csv_content = String::from("Index,Nombre,Carré\n");
    
    for (index, &number) in data.numbers.iter().enumerate() {
        csv_content.push_str(&format!("{},{},{}\n", index + 1, number, number * number));
    }
    
    Ok(csv_content)
}