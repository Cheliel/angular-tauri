# Tutoriel : Page Simple avec Tableau et Sauvegarde Tauri

## 🎯 **Objectif**

Créer une page Angular simple qui :
- Affiche un tableau de nombres
- Permet d'ajouter des chiffres
- Sauvegarde les données avec Tauri (fichier local)

---

## 🏗️ **Structure du Projet**

```
src/
├── app/
│   └── simple-table/
│       ├── simple-table.component.ts
│       ├── simple-table.component.html
│       ├── simple-table.component.scss
│       └── simple-table.component.spec.ts
└── ...

src-tauri/
├── src/
│   ├── main.rs
│   └── table_storage.rs
└── ...
```

---

## 📄 **1. Service Tauri**

### **📄 src/app/core/services/tauri/tauri.interfaces.ts**
```typescript
export interface NumberData {
  numbers: number[];
  created_at?: string;
  updated_at?: string;
}

export interface NumberStats {
  count: number;
  sum: number;
  average: number;
  min: number;
  max: number;
  created_at?: string;
  updated_at?: string;
}
```

### **📄 src/app/core/services/tauri/tauri.service.ts**
```typescript
import { Injectable } from '@angular/core';
import { invoke } from '@tauri-apps/api/core';
import { NumberData, NumberStats } from './tauri.interfaces';

@Injectable({
  providedIn: 'root'
})
export class TauriService {

  constructor() {}

  get isTauri(): boolean {
    return !!(window && window.__TAURI__);
  }

  // === GESTION DES NOMBRES ===
  async loadNumbers(): Promise<number[]> {
    try {
      const data: NumberData = await invoke('load_numbers');
      return data.numbers || [];
    } catch (error) {
      console.error('Erreur chargement nombres:', error);
      return [];
    }
  }

  async saveNumbers(numbers: number[]): Promise<boolean> {
    try {
      return await invoke('save_numbers', { numbers });
    } catch (error) {
      console.error('Erreur sauvegarde nombres:', error);
      throw error;
    }
  }

  async clearNumbers(): Promise<boolean> {
    try {
      return await invoke('clear_numbers');
    } catch (error) {
      console.error('Erreur effacement nombres:', error);
      throw error;
    }
  }

  async getNumbersStats(): Promise<NumberStats> {
    try {
      return await invoke('get_numbers_stats');
    } catch (error) {
      console.error('Erreur stats nombres:', error);
      throw error;
    }
  }

  async getNumbersInfo(): Promise<string> {
    try {
      return await invoke('get_numbers_info');
    } catch (error) {
      console.error('Erreur info nombres:', error);
      throw error;
    }
  }

  async clearAllData(): Promise<string> {
    try {
      return await invoke('clear_all_data');
    } catch (error) {
      console.error('Erreur suppression données:', error);
      throw error;
    }
  }
}
```

---

## 📄 **2. Composant Angular Frontend**

### **📄 src/app/simple-table/simple-table.component.ts**
```typescript
import { Component, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { RouterLink } from '@angular/router';
import { TauriService } from '../core/services/tauri/tauri.service';

@Component({
  selector: 'app-simple-table',
  templateUrl: './simple-table.component.html',
  styleUrls: ['./simple-table.component.scss'],
  standalone: true,
  imports: [CommonModule, FormsModule, RouterLink]
})
export class SimpleTableComponent implements OnInit {
  
  // Données du tableau
  numbers: number[] = [];
  
  // Nouveau nombre à ajouter
  newNumber: number | null = null;
  
  // États de l'interface
  loading = false;
  saving = false;
  message = '';

  constructor(private tauriService: TauriService) {}

  async ngOnInit() {
    await this.loadNumbers();
  }

  // Charger les données depuis Tauri
  async loadNumbers() {
    this.loading = true;
    this.message = 'Chargement des données...';
    
    try {
      if (this.tauriService.isTauri) {
        // Utiliser le service Tauri
        this.numbers = await this.tauriService.loadNumbers();
        this.message = `${this.numbers.length} nombres chargés depuis le fichier`;
      } else {
        // Mode navigateur - données de test
        this.numbers = [10, 25, 37, 42, 88];
        this.message = 'Mode navigateur - données de test';
      }
    } catch (error) {
      console.error('Erreur lors du chargement:', error);
      this.message = `Erreur: ${error}`;
      this.numbers = [];
    } finally {
      this.loading = false;
    }
  }

  // Ajouter un nouveau nombre
  async addNumber() {
    if (this.newNumber === null || this.newNumber === undefined) {
      this.message = 'Veuillez saisir un nombre valide';
      return;
    }

    // Ajouter à la liste locale
    this.numbers.push(this.newNumber);
    this.message = `Nombre ${this.newNumber} ajouté`;
    
    // Réinitialiser le champ
    this.newNumber = null;

    // Auto-sauvegarde
    await this.saveNumbers();
  }

  // Supprimer un nombre
  async removeNumber(index: number) {
    if (index >= 0 && index < this.numbers.length) {
      const removedNumber = this.numbers.splice(index, 1)[0];
      this.message = `Nombre ${removedNumber} supprimé`;
      await this.saveNumbers();
    }
  }

  // Sauvegarder via le service Tauri
  async saveNumbers() {
    this.saving = true;
    
    try {
      if (this.tauriService.isTauri) {
        await this.tauriService.saveNumbers(this.numbers);
        this.message = `${this.numbers.length} nombres sauvegardés`;
      } else {
        // Mode navigateur - simulation
        console.log('Sauvegarde simulée:', this.numbers);
        this.message = 'Sauvegarde simulée (mode navigateur)';
      }
    } catch (error) {
      console.error('Erreur lors de la sauvegarde:', error);
      this.message = `Erreur sauvegarde: ${error}`;
    } finally {
      this.saving = false;
    }
  }

  // Vider le tableau
  async clearAll() {
    if (confirm('Êtes-vous sûr de vouloir vider le tableau ?')) {
      this.numbers = [];
      this.message = 'Tableau vidé';
      
      if (this.tauriService.isTauri) {
        try {
          await this.tauriService.clearNumbers();
          this.message = 'Tableau vidé et sauvegardé';
        } catch (error) {
          console.error('Erreur lors de l\'effacement:', error);
          this.message = `Erreur effacement: ${error}`;
        }
      } else {
        this.message = 'Tableau vidé (mode navigateur)';
      }
    }
  }

  // Calculer la somme
  getSum(): number {
    return this.numbers.reduce((sum, num) => sum + num, 0);
  }

  // Calculer la moyenne
  getAverage(): number {
    return this.numbers.length > 0 ? this.getSum() / this.numbers.length : 0;
  }

  // Trouver le maximum
  getMax(): number {
    return this.numbers.length > 0 ? Math.max(...this.numbers) : 0;
  }

  // Trouver le minimum
  getMin(): number {
    return this.numbers.length > 0 ? Math.min(...this.numbers) : 0;
  }

  // Charger les statistiques depuis Tauri (optionnel)
  async loadStats() {
    if (this.tauriService.isTauri) {
      try {
        const stats = await this.tauriService.getNumbersStats();
        console.log('Statistiques depuis Tauri:', stats);
        this.message = `Stats chargées: ${stats.count} nombres, somme: ${stats.sum}`;
      } catch (error) {
        console.error('Erreur chargement stats:', error);
      }
    }
  }
}
```

### **📄 src/app/simple-table/simple-table.component.html**
```html
<div class="simple-table-container">
  
  <!-- En-tête -->
  <header class="page-header">
    <h1>📊 Tableau de Nombres</h1>
    <p class="subtitle">Ajoutez et gérez vos nombres avec sauvegarde automatique</p>
  </header>

  <!-- Formulaire d'ajout -->
  <section class="add-number-form">
    <div class="form-group">
      <label for="newNumber">Nouveau nombre :</label>
      <div class="input-group">
        <input 
          id="newNumber"
          type="number" 
          [(ngModel)]="newNumber" 
          placeholder="Saisissez un nombre"
          class="form-control"
          (keyup.enter)="addNumber()"
          [disabled]="saving">
        <button 
          (click)="addNumber()" 
          class="btn btn-primary"
          [disabled]="saving || newNumber === null || newNumber === undefined">
          <span *ngIf="saving">💾</span>
          <span *ngIf="!saving">➕</span>
          Ajouter
        </button>
      </div>
    </div>
  </section>

  <!-- Message de statut -->
  <div class="status-message" 
       [class.loading]="loading" 
       [class.saving]="saving"
       *ngIf="message">
    <span class="status-icon">
      <span *ngIf="loading">⏳</span>
      <span *ngIf="saving">💾</span>
      <span *ngIf="!loading && !saving">ℹ️</span>
    </span>
    {{ message }}
  </div>

  <!-- Statistiques -->
  <section class="statistics" *ngIf="numbers.length > 0">
    <div class="stats-grid">
      <div class="stat-card">
        <span class="stat-label">Total</span>
        <span class="stat-value">{{ numbers.length }}</span>
      </div>
      <div class="stat-card">
        <span class="stat-label">Somme</span>
        <span class="stat-value">{{ getSum() }}</span>
      </div>
      <div class="stat-card">
        <span class="stat-label">Moyenne</span>
        <span class="stat-value">{{ getAverage() | number:'1.1-2' }}</span>
      </div>
      <div class="stat-card">
        <span class="stat-label">Max</span>
        <span class="stat-value max">{{ getMax() }}</span>
      </div>
      <div class="stat-card">
        <span class="stat-label">Min</span>
        <span class="stat-value min">{{ getMin() }}</span>
      </div>
    </div>
  </section>

  <!-- Tableau des nombres -->
  <section class="numbers-table">
    <div class="table-header">
      <h2>Liste des Nombres ({{ numbers.length }})</h2>
      <div class="table-actions">
        <button (click)="loadNumbers()" 
                class="btn btn-secondary" 
                [disabled]="loading">
          🔄 Recharger
        </button>
        <button (click)="clearAll()" 
                class="btn btn-danger" 
                [disabled]="numbers.length === 0 || saving">
          🗑️ Vider
        </button>
      </div>
    </div>

    <!-- État vide -->
    <div class="empty-state" *ngIf="numbers.length === 0 && !loading">
      <div class="empty-icon">📝</div>
      <h3>Aucun nombre ajouté</h3>
      <p>Commencez par ajouter un nombre ci-dessus</p>
    </div>

    <!-- État de chargement -->
    <div class="loading-state" *ngIf="loading">
      <div class="spinner"></div>
      <p>Chargement en cours...</p>
    </div>

    <!-- Tableau -->
    <div class="table-wrapper" *ngIf="numbers.length > 0 && !loading">
      <table class="numbers-table-grid">
        <thead>
          <tr>
            <th>Index</th>
            <th>Valeur</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr *ngFor="let number of numbers; let i = index" 
              [class.highlight]="number === getMax() || number === getMin()">
            <td class="index-cell">{{ i + 1 }}</td>
            <td class="number-cell" 
                [class.max]="number === getMax()" 
                [class.min]="number === getMin()">
              {{ number }}
            </td>
            <td class="actions-cell">
              <button (click)="removeNumber(i)" 
                      class="btn btn-sm btn-danger"
                      [disabled]="saving"
                      title="Supprimer ce nombre">
                ❌
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </section>

  <!-- Navigation -->
  <nav class="page-navigation">
    <a routerLink="/home" class="nav-link">
      🏠 Retour à l'accueil
    </a>
  </nav>

</div>
```

### **📄 src/app/simple-table/simple-table.component.scss**
```scss
.simple-table-container {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
  background-color: #f8f9fa;
  min-height: 100vh;
}

.page-header {
  text-align: center;
  margin-bottom: 30px;
  padding: 30px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-radius: 12px;
  color: white;
  box-shadow: 0 4px 15px rgba(0,0,0,0.1);

  h1 {
    margin: 0 0 10px 0;
    font-size: 2.5rem;
    font-weight: 600;
  }

  .subtitle {
    margin: 0;
    opacity: 0.9;
    font-size: 1.1rem;
  }
}

.add-number-form {
  background: white;
  padding: 25px;
  border-radius: 12px;
  box-shadow: 0 2px 10px rgba(0,0,0,0.08);
  margin-bottom: 20px;

  .form-group {
    label {
      display: block;
      margin-bottom: 8px;
      font-weight: 600;
      color: #333;
    }
  }

  .input-group {
    display: flex;
    gap: 10px;
    align-items: center;

    .form-control {
      flex: 1;
      padding: 12px 16px;
      border: 2px solid #e9ecef;
      border-radius: 8px;
      font-size: 16px;
      transition: border-color 0.3s ease, box-shadow 0.3s ease;

      &:focus {
        outline: none;
        border-color: #667eea;
        box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
      }

      &:disabled {
        background-color: #f8f9fa;
        cursor: not-allowed;
      }
    }
  }
}

.btn {
  padding: 12px 20px;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-weight: 600;
  transition: all 0.3s ease;
  display: inline-flex;
  align-items: center;
  gap: 8px;

  &.btn-primary {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;

    &:hover:not(:disabled) {
      transform: translateY(-1px);
      box-shadow: 0 4px 15px rgba(102, 126, 234, 0.4);
    }
  }

  &.btn-secondary {
    background-color: #6c757d;
    color: white;

    &:hover:not(:disabled) {
      background-color: #545b62;
    }
  }

  &.btn-danger {
    background-color: #dc3545;
    color: white;

    &:hover:not(:disabled) {
      background-color: #c82333;
    }
  }

  &.btn-sm {
    padding: 6px 12px;
    font-size: 14px;
  }

  &:disabled {
    opacity: 0.6;
    cursor: not-allowed;
    transform: none !important;
    box-shadow: none !important;
  }
}

.status-message {
  padding: 15px 20px;
  border-radius: 8px;
  margin-bottom: 20px;
  background-color: #d4edda;
  border: 1px solid #c3e6cb;
  color: #155724;
  display: flex;
  align-items: center;
  gap: 10px;

  &.loading {
    background-color: #cce7ff;
    border-color: #99d6ff;
    color: #004085;
  }

  &.saving {
    background-color: #fff3cd;
    border-color: #ffeaa7;
    color: #856404;
  }

  .status-icon {
    font-size: 1.2rem;
  }
}

.statistics {
  background: white;
  padding: 25px;
  border-radius: 12px;
  box-shadow: 0 2px 10px rgba(0,0,0,0.08);
  margin-bottom: 20px;

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
    gap: 15px;
  }

  .stat-card {
    text-align: center;
    padding: 20px;
    background: linear-gradient(135deg, #f8f9fa 0%, #e9ecef 100%);
    border-radius: 8px;
    transition: transform 0.3s ease;

    &:hover {
      transform: translateY(-2px);
    }

    .stat-label {
      display: block;
      font-size: 0.9rem;
      color: #6c757d;
      margin-bottom: 5px;
    }

    .stat-value {
      display: block;
      font-size: 1.8rem;
      font-weight: 700;
      color: #333;

      &.max {
        color: #28a745;
      }

      &.min {
        color: #dc3545;
      }
    }
  }
}

.numbers-table {
  background: white;
  border-radius: 12px;
  box-shadow: 0 2px 10px rgba(0,0,0,0.08);
  overflow: hidden;
  margin-bottom: 30px;

  .table-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 20px 25px;
    background-color: #f8f9fa;
    border-bottom: 1px solid #dee2e6;

    h2 {
      margin: 0;
      color: #333;
    }

    .table-actions {
      display: flex;
      gap: 10px;
    }
  }
}

.empty-state {
  text-align: center;
  padding: 60px 20px;
  color: #6c757d;

  .empty-icon {
    font-size: 4rem;
    margin-bottom: 20px;
  }

  h3 {
    margin-bottom: 10px;
    color: #495057;
  }

  p {
    margin: 0;
  }
}

.loading-state {
  text-align: center;
  padding: 60px 20px;

  .spinner {
    width: 50px;
    height: 50px;
    border: 4px solid #f3f3f3;
    border-top: 4px solid #667eea;
    border-radius: 50%;
    animation: spin 1s linear infinite;
    margin: 0 auto 20px;
  }
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.table-wrapper {
  padding: 0;
  overflow-x: auto;
}

.numbers-table-grid {
  width: 100%;
  border-collapse: collapse;

  th {
    background-color: #f8f9fa;
    padding: 15px;
    text-align: left;
    font-weight: 600;
    color: #333;
    border-bottom: 2px solid #dee2e6;
  }

  td {
    padding: 15px;
    border-bottom: 1px solid #dee2e6;
    vertical-align: middle;
  }

  .index-cell {
    width: 80px;
    text-align: center;
    font-weight: 600;
    color: #6c757d;
  }

  .number-cell {
    font-size: 1.2rem;
    font-weight: 600;
    text-align: center;

    &.max {
      background-color: rgba(40, 167, 69, 0.1);
      color: #28a745;
    }

    &.min {
      background-color: rgba(220, 53, 69, 0.1);
      color: #dc3545;
    }
  }

  .actions-cell {
    width: 100px;
    text-align: center;
  }

  tr {
    transition: background-color 0.2s ease;

    &:hover {
      background-color: #f8f9fa;
    }

    &.highlight {
      background-color: rgba(255, 193, 7, 0.1);
    }
  }
}

.page-navigation {
  text-align: center;
  padding: 20px 0;

  .nav-link {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 12px 24px;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    text-decoration: none;
    border-radius: 25px;
    font-weight: 600;
    transition: all 0.3s ease;

    &:hover {
      transform: translateY(-2px);
      box-shadow: 0 6px 20px rgba(102, 126, 234, 0.4);
    }
  }
}

// Responsive Design
@media (max-width: 768px) {
  .simple-table-container {
    padding: 15px;
  }

  .page-header {
    padding: 20px;

    h1 {
      font-size: 2rem;
    }

    .subtitle {
      font-size: 1rem;
    }
  }

  .input-group {
    flex-direction: column;
    align-items: stretch;
  }

  .table-header {
    flex-direction: column;
    gap: 15px;
    text-align: center;
  }

  .stats-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}

@media (max-width: 480px) {
  .stats-grid {
    grid-template-columns: 1fr;
  }
}
```

---

## 🦀 **3. Backend Rust (Tauri)**

### **📄 src-tauri/src/table_storage.rs**
```rust
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
```

### **📄 src-tauri/src/main.rs** (Mise à jour)
```rust
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
```

### **📄 src-tauri/Cargo.toml** (Dépendances nécessaires)
```toml
[package]
name = "angular-tauri"
version = "1.0.0"
description = "Application simple avec tableau de nombres"
authors = ["Votre Nom <email@example.com>"]
license = ""
repository = ""
edition = "2021"

[build-dependencies]
tauri-build = { version = "1.5", features = [] }

[dependencies]
tauri = { version = "1.7", features = ["api-all"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }

[features]
# Cette fonctionnalité est utilisée pour les builds de production
# NE PAS SUPPRIMER !!
custom-protocol = ["tauri/custom-protocol"]
```

---

## 🛣️ **4. Configuration des Routes**

### **📄 src/app/app.routes.ts**
```typescript
import { Routes } from '@angular/router';
import { HomeComponent } from './home/home.component';

export const routes: Routes = [
  {
    path: '',
    redirectTo: '/home',
    pathMatch: 'full'
  },
  {
    path: 'home',
    component: HomeComponent,
    title: 'Accueil'
  },
  {
    path: 'simple-table',
    loadComponent: () => import('./simple-table/simple-table.component')
      .then(m => m.SimpleTableComponent),
    title: 'Tableau Simple'
  },
  {
    path: '**',
    redirectTo: '/home'
  }
];
```

### **📄 src/app/home/home.component.html** (Ajouter le lien)
```html
<div class="container">
  <h1 class="title">
    {{ 'PAGES.HOME.TITLE' | translate }}
  </h1>

  <nav class="page-links">
    <a routerLink="/detail" class="page-link">
      {{ 'PAGES.HOME.GO_TO_DETAIL' | translate }}
    </a>
    
    <a routerLink="/simple-table" class="page-link primary">
      📊 Tableau de Nombres
    </a>
  </nav>
</div>
```

### **📄 src/app/home/home.component.scss** (Style pour les liens)
```scss
:host {
  display: block;
  padding: 20px;
}

.container {
  max-width: 600px;
  margin: 0 auto;
  text-align: center;
}

.title {
  font-size: 2.5rem;
  color: #333;
  margin-bottom: 40px;
}

.page-links {
  display: flex;
  flex-direction: column;
  gap: 20px;
  align-items: center;
}

.page-link {
  display: inline-flex;
  align-items: center;
  padding: 15px 30px;
  text-decoration: none;
  border-radius: 25px;
  font-weight: 600;
  transition: all 0.3s ease;
  min-width: 200px;
  justify-content: center;

  // Lien par défaut
  background-color: #6c757d;
  color: white;
  border: 2px solid #6c757d;

  &:hover {
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(108, 117, 125, 0.4);
  }

  // Lien principal
  &.primary {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    border-color: #667eea;
    font-size: 1.1rem;

    &:hover {
      box-shadow: 0 6px 20px rgba(102, 126, 234, 0.4);
    }
  }
}
```

---

## 🧪 **5. Tests**

### **📄 src/app/simple-table/simple-table.component.spec.ts**
```typescript
import { ComponentFixture, TestBed } from '@angular/core/testing';
import { SimpleTableComponent } from './simple-table.component';
import { TauriService } from '../core/services/tauri/tauri.service';
import { provideRouter } from '@angular/router';

describe('SimpleTableComponent', () => {
  let component: SimpleTableComponent;
  let fixture: ComponentFixture<SimpleTableComponent>;
  let mockTauriService: jasmine.SpyObj<TauriService>;

  beforeEach(async () => {
    const spy = jasmine.createSpyObj('TauriService', [
      'loadNumbers', 
      'saveNumbers', 
      'clearNumbers', 
      'getNumbersStats'
    ], {
      isTauri: false
    });

    await TestBed.configureTestingModule({
      imports: [SimpleTableComponent],
      providers: [
        provideRouter([]),
        { provide: TauriService, useValue: spy }
      ]
    }).compileComponents();

    fixture = TestBed.createComponent(SimpleTableComponent);
    component = fixture.componentInstance;
    mockTauriService = TestBed.inject(TauriService) as jasmine.SpyObj<TauriService>;
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should load numbers on init', async () => {
    mockTauriService.loadNumbers.and.returnValue(Promise.resolve([1, 2, 3]));
    
    await component.ngOnInit();
    
    expect(mockTauriService.loadNumbers).toHaveBeenCalled();
    expect(component.numbers).toEqual([1, 2, 3]);
  });

  it('should add number to array', async () => {
    mockTauriService.saveNumbers.and.returnValue(Promise.resolve(true));
    component.newNumber = 42;
    
    await component.addNumber();
    
    expect(component.numbers).toContain(42);
    expect(mockTauriService.saveNumbers).toHaveBeenCalledWith([42]);
  });

  it('should calculate sum correctly', () => {
    component.numbers = [10, 20, 30];
    expect(component.getSum()).toBe(60);
  });

  it('should calculate average correctly', () => {
    component.numbers = [10, 20, 30];
    expect(component.getAverage()).toBe(20);
  });

  it('should find max value', () => {
    component.numbers = [5, 15, 3, 22, 8];
    expect(component.getMax()).toBe(22);
  });

  it('should find min value', () => {
    component.numbers = [5, 15, 3, 22, 8];
    expect(component.getMin()).toBe(3);
  });

  it('should remove number at index', async () => {
    mockTauriService.saveNumbers.and.returnValue(Promise.resolve(true));
    component.numbers = [10, 20, 30];
    
    await component.removeNumber(1);
    
    expect(component.numbers).toEqual([10, 30]);
    expect(mockTauriService.saveNumbers).toHaveBeenCalledWith([10, 30]);
  });

  it('should clear all numbers with Tauri service', async () => {
    mockTauriService.clearNumbers.and.returnValue(Promise.resolve(true));
    spyOn(window, 'confirm').and.returnValue(true);
    component.numbers = [10, 20, 30];
    
    await component.clearAll();
    
    expect(component.numbers.length).toBe(0);
    expect(mockTauriService.clearNumbers).toHaveBeenCalled();
  });
});
```

---

## ⚙️ **6. Configuration Tauri**

### **📄 src-tauri/tauri.conf.json**
```json
{
  "$schema": "../node_modules/@tauri-apps/cli/schema.json",
  "build": {
    "beforeBuildCommand": "npm run build",
    "beforeDevCommand": "npm run serve",
    "devPath": "http://localhost:4200",
    "distDir": "../dist/angular-tauri"
  },
  "package": {
    "productName": "Tableau Simple",
    "version": "1.0.0"
  },
  "tauri": {
    "allowlist": {
      "all": false,
      "fs": {
        "all": false,
        "readFile": true,
        "writeFile": true,
        "createDir": true,
        "scope": ["$APPDATA/*"]
      },
      "path": {
        "all": true
      }
    },
    "bundle": {
      "active": true,
      "category": "papi-mami",
      "copyright": "",
      "identifier": "com.example.simple-table",
      "longDescription": "Application simple pour gérer un tableau de nombres avec sauvegarde locale",
      "shortDescription": "Tableau Simple",
      "targets": "all"
    },
    "security": {
      "csp": null
    },
    "windows": [
      {
        "fullscreen": false,
        "height": 700,
        "resizable": true,
        "title": "Tableau Simple",
        "width": 900,
        "minWidth": 500,
        "minHeight": 400
      }
    ]
  }
}
```

---

## 🚀 **7. Utilisation**

### **Commandes de développement**
```bash
# Développement avec hot reload
npm run tauri:dev

# Build de production
npm run tauri:build

# Tests
npm run test
```

### **Fonctionnalités**
- ✅ **Ajout de nombres** : Saisir et ajouter des nombres au tableau
- ✅ **Suppression** : Retirer des nombres individuellement
- ✅ **Statistiques** : Somme, moyenne, min, max automatiques
- ✅ **Sauvegarde automatique** : Persistance des données avec Tauri
- ✅ **Interface responsive** : Adaptée mobile et desktop
- ✅ **Gestion d'erreurs** : Messages d'état et gestion des cas d'erreur
- ✅ **Mode navigateur** : Fonctionne aussi sans Tauri (données test)

### **Fichier de sauvegarde**
Les données sont sauvegardées dans :
- **Windows** : `%APPDATA%/com.example.simple-table/numbers_data.json`
- **macOS** : `~/Library/Application Support/com.example.simple-table/numbers_data.json`
- **Linux** : `~/.local/share/com.example.simple-table/numbers_data.json`

---

## 🎯 **Résumé**

Ce tutoriel vous donne une application complète et fonctionnelle avec :

🔧 **Service Angular** : TauriService centralisé pour toute la communication avec Tauri

🚀 **Frontend Angular** : Composant standalone qui utilise le service au lieu des appels invoke directs

🦀 **Backend Rust/Tauri** : Commandes adaptées aux méthodes du service avec types cohérents

🎨 **Design moderne** : Interface responsive avec animations et états visuels

🧪 **Tests inclus** : Tests unitaires avec mocks du service Tauri

📱 **Cross-platform** : Fonctionne sur Windows, macOS, et Linux

### **🔄 Avantages de cette architecture :**

✅ **Centralisation** : Toutes les appels Tauri dans un seul service

✅ **Types sécurisés** : Interfaces TypeScript pour les données

✅ **Testabilité** : Service facilement mockable pour les tests

✅ **Maintenance** : Modifications centralisées dans le service

✅ **Réutilisabilité** : Service utilisable dans tous les composants

Une architecture propre et maintenable pour vos applications Angular + Tauri ! 🚀