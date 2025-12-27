# Tutoriel Angular + Tauri : Application de Comptabilité

## 📖 **Table des Matières**
1. [Structure du Projet](#structure-du-projet)
2. [Communication Angular ↔ Tauri](#communication-angular--tauri)
3. [Base de Données SQLite Locale](#base-de-données-sqlite-locale)
4. [Fonctionnalités Comptabilité](#fonctionnalités-comptabilité)
5. [Sécurité et Permissions](#sécurité-et-permissions)
6. [Démarrage et Build](#démarrage-et-build)

---

## 🏗️ **Structure du Projet**

### **Vue d'Ensemble**
```
angular-tauri/
├── 📁 src/                          ← Frontend Angular
│   ├── 📁 app/
│   │   ├── 📁 core/
│   │   │   └── 📁 services/
│   │   │       └── 📁 tauri/        ← Service communication Tauri
│   │   ├── 📁 accounting/           ← Modules comptabilité
│   │   │   ├── 📁 components/
│   │   │   ├── 📁 services/
│   │   │   └── 📁 models/
│   │   └── 📁 shared/
│   ├── 📄 index.html
│   └── 📄 main.ts
├── 📄 package.json
└── 📄 angular.json

src-tauri/                           ← Backend Rust
├── 📁 src/
│   ├── 📄 main.rs                   ← Point d'entrée principal
│   ├── 📄 database.rs               ← Module SQLite
│   ├── 📄 accounting.rs             ← Commandes comptabilité
│   └── 📄 lib.rs
├── 📄 Cargo.toml                    ← Dépendances Rust
├── 📄 tauri.conf.json               ← Configuration Tauri
└── 📁 icons/
```

---

## 🔄 **Communication Angular ↔ Tauri**

### **1. Service Angular (Frontend)**

#### **📄 src/app/core/services/tauri/tauri.service.ts**
```typescript
import { Injectable } from '@angular/core';
import { invoke } from '@tauri-apps/api/tauri';

export interface Transaction {
  id?: number;
  date: string;
  description: string;
  amount: number;
  category: string;
  type: 'income' | 'expense';
}

export interface Account {
  id?: number;
  name: string;
  balance: number;
  type: 'asset' | 'liability' | 'equity';
}

@Injectable({
  providedIn: 'root'
})
export class TauriService {

  // === GESTION BASE DE DONNÉES ===
  async initDatabase(): Promise<boolean> {
    try {
      return await invoke('init_database');
    } catch (error) {
      console.error('Erreur initialisation DB:', error);
      throw error;
    }
  }

  // === GESTION TRANSACTIONS ===
  async createTransaction(transaction: Transaction): Promise<number> {
    return await invoke('create_transaction', { transaction });
  }

  async getTransactions(limit?: number): Promise<Transaction[]> {
    return await invoke('get_transactions', { limit: limit || 100 });
  }

  async updateTransaction(transaction: Transaction): Promise<boolean> {
    return await invoke('update_transaction', { transaction });
  }

  async deleteTransaction(id: number): Promise<boolean> {
    return await invoke('delete_transaction', { id });
  }

  // === GESTION COMPTES ===
  async createAccount(account: Account): Promise<number> {
    return await invoke('create_account', { account });
  }

  async getAccounts(): Promise<Account[]> {
    return await invoke('get_accounts');
  }

  async updateAccount(account: Account): Promise<boolean> {
    return await invoke('update_account', { account });
  }

  // === RAPPORTS ===
  async getMonthlyReport(year: number, month: number): Promise<any> {
    return await invoke('get_monthly_report', { year, month });
  }

  async exportData(format: 'csv' | 'json'): Promise<string> {
    return await invoke('export_data', { format });
  }

  // === FICHIERS ===
  async importCsv(filePath: string): Promise<number> {
    return await invoke('import_csv', { filePath });
  }

  async backupDatabase(): Promise<string> {
    return await invoke('backup_database');
  }

  async restoreDatabase(backupPath: string): Promise<boolean> {
    return await invoke('restore_database', { backupPath });
  }
}
```

### **2. Backend Rust (Tauri)**

#### **📄 src-tauri/src/main.rs**
```rust
// Prevent console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod accounting;

use database::Database;
use accounting::*;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Initialiser la base de données au démarrage
            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = Database::init(&app_handle).await {
                    eprintln!("Erreur initialisation DB: {}", e);
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            init_database,
            create_transaction,
            get_transactions,
            update_transaction,
            delete_transaction,
            create_account,
            get_accounts,
            update_account,
            get_monthly_report,
            export_data,
            import_csv,
            backup_database,
            restore_database
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

#### **📄 src-tauri/src/database.rs**
```rust
use rusqlite::{Connection, Result, params};
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Option<i64>,
    pub date: String,
    pub description: String,
    pub amount: f64,
    pub category: String,
    pub transaction_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: Option<i64>,
    pub name: String,
    pub balance: f64,
    pub account_type: String,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub async fn init(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
        let app_dir = app.path_resolver()
            .app_data_dir()
            .expect("failed to resolve app data dir");
        
        std::fs::create_dir_all(&app_dir)?;
        let db_path = app_dir.join("accounting.db");
        
        let conn = Connection::open(db_path)?;
        
        // Créer les tables
        conn.execute(
            "CREATE TABLE IF NOT EXISTS transactions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                date TEXT NOT NULL,
                description TEXT NOT NULL,
                amount REAL NOT NULL,
                category TEXT NOT NULL,
                transaction_type TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS accounts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                balance REAL NOT NULL DEFAULT 0,
                account_type TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        // Créer des comptes par défaut si nécessaire
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM accounts",
            [],
            |row| row.get(0)
        )?;

        if count == 0 {
            conn.execute(
                "INSERT INTO accounts (name, balance, account_type) VALUES 
                ('Compte Principal', 0, 'asset'),
                ('Revenus', 0, 'equity'),
                ('Dépenses', 0, 'expense')",
                [],
            )?;
        }

        Ok(())
    }

    pub fn get_connection(app: &AppHandle) -> Result<Connection, Box<dyn std::error::Error>> {
        let app_dir = app.path_resolver()
            .app_data_dir()
            .expect("failed to resolve app data dir");
        let db_path = app_dir.join("accounting.db");
        Ok(Connection::open(db_path)?)
    }
}
```

#### **📄 src-tauri/src/accounting.rs**
```rust
use crate::database::{Database, Transaction, Account};
use rusqlite::{params, Connection};
use serde_json::Value;
use tauri::{command, AppHandle, State};
use std::collections::HashMap;

// === COMMANDES DATABASE ===
#[command]
pub async fn init_database(app: AppHandle) -> Result<bool, String> {
    Database::init(&app).await.map_err(|e| e.to_string())?;
    Ok(true)
}

// === COMMANDES TRANSACTIONS ===
#[command]
pub async fn create_transaction(
    app: AppHandle,
    transaction: Transaction,
) -> Result<i64, String> {
    let conn = Database::get_connection(&app).map_err(|e| e.to_string())?;
    
    conn.execute(
        "INSERT INTO transactions (date, description, amount, category, transaction_type)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            transaction.date,
            transaction.description,
            transaction.amount,
            transaction.category,
            transaction.transaction_type
        ],
    ).map_err(|e| e.to_string())?;

    Ok(conn.last_insert_rowid())
}

#[command]
pub async fn get_transactions(
    app: AppHandle,
    limit: Option<i64>,
) -> Result<Vec<Transaction>, String> {
    let conn = Database::get_connection(&app).map_err(|e| e.to_string())?;
    let limit = limit.unwrap_or(100);
    
    let mut stmt = conn.prepare(
        "SELECT id, date, description, amount, category, transaction_type 
         FROM transactions ORDER BY date DESC, id DESC LIMIT ?1"
    ).map_err(|e| e.to_string())?;

    let transaction_iter = stmt.query_map(params![limit], |row| {
        Ok(Transaction {
            id: Some(row.get(0)?),
            date: row.get(1)?,
            description: row.get(2)?,
            amount: row.get(3)?,
            category: row.get(4)?,
            transaction_type: row.get(5)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut transactions = Vec::new();
    for transaction in transaction_iter {
        transactions.push(transaction.map_err(|e| e.to_string())?);
    }

    Ok(transactions)
}

#[command]
pub async fn update_transaction(
    app: AppHandle,
    transaction: Transaction,
) -> Result<bool, String> {
    let conn = Database::get_connection(&app).map_err(|e| e.to_string())?;
    
    let rows = conn.execute(
        "UPDATE transactions SET date=?1, description=?2, amount=?3, category=?4, transaction_type=?5
         WHERE id=?6",
        params![
            transaction.date,
            transaction.description,
            transaction.amount,
            transaction.category,
            transaction.transaction_type,
            transaction.id.unwrap_or(0)
        ],
    ).map_err(|e| e.to_string())?;

    Ok(rows > 0)
}

#[command]
pub async fn delete_transaction(app: AppHandle, id: i64) -> Result<bool, String> {
    let conn = Database::get_connection(&app).map_err(|e| e.to_string())?;
    
    let rows = conn.execute(
        "DELETE FROM transactions WHERE id = ?1",
        params![id],
    ).map_err(|e| e.to_string())?;

    Ok(rows > 0)
}

// === COMMANDES COMPTES ===
#[command]
pub async fn create_account(app: AppHandle, account: Account) -> Result<i64, String> {
    let conn = Database::get_connection(&app).map_err(|e| e.to_string())?;
    
    conn.execute(
        "INSERT INTO accounts (name, balance, account_type) VALUES (?1, ?2, ?3)",
        params![account.name, account.balance, account.account_type],
    ).map_err(|e| e.to_string())?;

    Ok(conn.last_insert_rowid())
}

#[command]
pub async fn get_accounts(app: AppHandle) -> Result<Vec<Account>, String> {
    let conn = Database::get_connection(&app).map_err(|e| e.to_string())?;
    
    let mut stmt = conn.prepare(
        "SELECT id, name, balance, account_type FROM accounts ORDER BY name"
    ).map_err(|e| e.to_string())?;

    let account_iter = stmt.query_map([], |row| {
        Ok(Account {
            id: Some(row.get(0)?),
            name: row.get(1)?,
            balance: row.get(2)?,
            account_type: row.get(3)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut accounts = Vec::new();
    for account in account_iter {
        accounts.push(account.map_err(|e| e.to_string())?);
    }

    Ok(accounts)
}

#[command]
pub async fn update_account(app: AppHandle, account: Account) -> Result<bool, String> {
    let conn = Database::get_connection(&app).map_err(|e| e.to_string())?;
    
    let rows = conn.execute(
        "UPDATE accounts SET name=?1, balance=?2, account_type=?3 WHERE id=?4",
        params![
            account.name,
            account.balance,
            account.account_type,
            account.id.unwrap_or(0)
        ],
    ).map_err(|e| e.to_string())?;

    Ok(rows > 0)
}

// === RAPPORTS ===
#[command]
pub async fn get_monthly_report(
    app: AppHandle,
    year: i32,
    month: i32,
) -> Result<Value, String> {
    let conn = Database::get_connection(&app).map_err(|e| e.to_string())?;
    
    let start_date = format!("{:04}-{:02}-01", year, month);
    let end_date = if month == 12 {
        format!("{:04}-01-01", year + 1)
    } else {
        format!("{:04}-{:02}-01", year, month + 1)
    };

    // Total des revenus
    let income: f64 = conn.query_row(
        "SELECT COALESCE(SUM(amount), 0) FROM transactions 
         WHERE transaction_type = 'income' AND date >= ?1 AND date < ?2",
        params![start_date, end_date],
        |row| row.get(0)
    ).map_err(|e| e.to_string())?;

    // Total des dépenses
    let expenses: f64 = conn.query_row(
        "SELECT COALESCE(SUM(amount), 0) FROM transactions 
         WHERE transaction_type = 'expense' AND date >= ?1 AND date < ?2",
        params![start_date, end_date],
        |row| row.get(0)
    ).map_err(|e| e.to_string())?;

    // Dépenses par catégorie
    let mut stmt = conn.prepare(
        "SELECT category, SUM(amount) FROM transactions 
         WHERE transaction_type = 'expense' AND date >= ?1 AND date < ?2
         GROUP BY category ORDER BY SUM(amount) DESC"
    ).map_err(|e| e.to_string())?;

    let category_iter = stmt.query_map(params![start_date, end_date], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, f64>(1)?))
    }).map_err(|e| e.to_string())?;

    let mut categories = HashMap::new();
    for category in category_iter {
        let (cat, amount) = category.map_err(|e| e.to_string())?;
        categories.insert(cat, amount);
    }

    Ok(serde_json::json!({
        "period": format!("{:04}-{:02}", year, month),
        "income": income,
        "expenses": expenses,
        "balance": income - expenses,
        "categories": categories
    }))
}

// === IMPORT/EXPORT ===
#[command]
pub async fn export_data(app: AppHandle, format: String) -> Result<String, String> {
    let conn = Database::get_connection(&app).map_err(|e| e.to_string())?;
    
    match format.as_str() {
        "csv" => {
            let mut stmt = conn.prepare(
                "SELECT date, description, amount, category, transaction_type 
                 FROM transactions ORDER BY date DESC"
            ).map_err(|e| e.to_string())?;

            let mut csv_content = String::from("Date,Description,Amount,Category,Type\n");
            
            let rows = stmt.query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, f64>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                ))
            }).map_err(|e| e.to_string())?;

            for row in rows {
                let (date, desc, amount, category, t_type) = row.map_err(|e| e.to_string())?;
                csv_content.push_str(&format!("{},{},{},{},{}\n", date, desc, amount, category, t_type));
            }

            Ok(csv_content)
        },
        "json" => {
            let transactions = get_transactions(app, None).await?;
            serde_json::to_string_pretty(&transactions).map_err(|e| e.to_string())
        },
        _ => Err("Format non supporté".to_string())
    }
}

#[command]
pub async fn backup_database(app: AppHandle) -> Result<String, String> {
    use std::fs;
    use chrono::Utc;
    
    let app_dir = app.path_resolver()
        .app_data_dir()
        .expect("failed to resolve app data dir");
    
    let db_path = app_dir.join("accounting.db");
    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
    let backup_path = app_dir.join(format!("backup_accounting_{}.db", timestamp));
    
    fs::copy(&db_path, &backup_path).map_err(|e| e.to_string())?;
    
    Ok(backup_path.to_string_lossy().to_string())
}

#[command]
pub async fn restore_database(app: AppHandle, backup_path: String) -> Result<bool, String> {
    use std::fs;
    
    let app_dir = app.path_resolver()
        .app_data_dir()
        .expect("failed to resolve app data dir");
    
    let db_path = app_dir.join("accounting.db");
    
    fs::copy(&backup_path, &db_path).map_err(|e| e.to_string())?;
    
    Ok(true)
}
```

---

## 📊 **Fonctionnalités Comptabilité**

### **1. Component de Gestion des Transactions**

#### **📄 src/app/accounting/components/transaction-form/transaction-form.component.ts**
```typescript
import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { TauriService, Transaction } from '../../../core/services/tauri/tauri.service';

@Component({
  selector: 'app-transaction-form',
  template: `
    <div class="transaction-form">
      <h3>{{ editMode ? 'Modifier' : 'Nouvelle' }} Transaction</h3>
      
      <form [formGroup]="transactionForm" (ngSubmit)="onSubmit()">
        <div class="form-group">
          <label for="date">Date</label>
          <input type="date" id="date" formControlName="date" class="form-control">
        </div>

        <div class="form-group">
          <label for="type">Type</label>
          <select id="type" formControlName="type" class="form-control">
            <option value="income">Revenu</option>
            <option value="expense">Dépense</option>
          </select>
        </div>

        <div class="form-group">
          <label for="category">Catégorie</label>
          <select id="category" formControlName="category" class="form-control">
            <option *ngFor="let cat of categories" [value]="cat">{{cat}}</option>
          </select>
        </div>

        <div class="form-group">
          <label for="description">Description</label>
          <input type="text" id="description" formControlName="description" 
                 class="form-control" placeholder="Description de la transaction">
        </div>

        <div class="form-group">
          <label for="amount">Montant (€)</label>
          <input type="number" id="amount" formControlName="amount" 
                 step="0.01" class="form-control" placeholder="0.00">
        </div>

        <div class="form-actions">
          <button type="submit" [disabled]="!transactionForm.valid" 
                  class="btn btn-primary">
            {{ editMode ? 'Modifier' : 'Ajouter' }}
          </button>
          <button type="button" (click)="onReset()" class="btn btn-secondary">
            Annuler
          </button>
        </div>
      </form>
    </div>
  `,
  styles: [`
    .transaction-form {
      max-width: 500px;
      margin: 0 auto;
      padding: 20px;
      border: 1px solid #ddd;
      border-radius: 8px;
    }
    .form-group {
      margin-bottom: 15px;
    }
    .form-control {
      width: 100%;
      padding: 8px 12px;
      border: 1px solid #ccc;
      border-radius: 4px;
    }
    .form-actions {
      display: flex;
      gap: 10px;
      justify-content: flex-end;
    }
    .btn {
      padding: 8px 16px;
      border: none;
      border-radius: 4px;
      cursor: pointer;
    }
    .btn-primary {
      background-color: #007bff;
      color: white;
    }
    .btn-secondary {
      background-color: #6c757d;
      color: white;
    }
    .btn:disabled {
      opacity: 0.6;
      cursor: not-allowed;
    }
  `]
})
export class TransactionFormComponent implements OnInit {
  transactionForm: FormGroup;
  editMode = false;
  currentTransaction?: Transaction;
  
  categories = [
    'Alimentation', 'Transport', 'Logement', 'Santé', 'Loisirs',
    'Vêtements', 'Éducation', 'Services', 'Autres', 'Salaire',
    'Freelance', 'Investissements', 'Cadeaux'
  ];

  constructor(
    private fb: FormBuilder,
    private tauriService: TauriService
  ) {
    this.transactionForm = this.fb.group({
      date: [new Date().toISOString().split('T')[0], Validators.required],
      type: ['expense', Validators.required],
      category: ['', Validators.required],
      description: ['', Validators.required],
      amount: [0, [Validators.required, Validators.min(0.01)]]
    });
  }

  ngOnInit() {
    // Initialiser la base de données
    this.tauriService.initDatabase();
  }

  async onSubmit() {
    if (this.transactionForm.valid) {
      const formValue = this.transactionForm.value;
      const transaction: Transaction = {
        id: this.currentTransaction?.id,
        date: formValue.date,
        description: formValue.description,
        amount: parseFloat(formValue.amount),
        category: formValue.category,
        type: formValue.type
      };

      try {
        if (this.editMode && this.currentTransaction?.id) {
          await this.tauriService.updateTransaction(transaction);
          alert('Transaction modifiée avec succès !');
        } else {
          await this.tauriService.createTransaction(transaction);
          alert('Transaction ajoutée avec succès !');
        }
        this.onReset();
      } catch (error) {
        alert('Erreur lors de la sauvegarde : ' + error);
      }
    }
  }

  onReset() {
    this.transactionForm.reset({
      date: new Date().toISOString().split('T')[0],
      type: 'expense',
      category: '',
      description: '',
      amount: 0
    });
    this.editMode = false;
    this.currentTransaction = undefined;
  }

  editTransaction(transaction: Transaction) {
    this.currentTransaction = transaction;
    this.editMode = true;
    this.transactionForm.patchValue({
      date: transaction.date,
      type: transaction.type,
      category: transaction.category,
      description: transaction.description,
      amount: transaction.amount
    });
  }
}
```

### **2. Component Liste des Transactions**

#### **📄 src/app/accounting/components/transaction-list/transaction-list.component.ts**
```typescript
import { Component, OnInit } from '@angular/core';
import { TauriService, Transaction } from '../../../core/services/tauri/tauri.service';

@Component({
  selector: 'app-transaction-list',
  template: `
    <div class="transaction-list">
      <div class="header">
        <h3>Historique des Transactions</h3>
        <div class="actions">
          <button (click)="refreshTransactions()" class="btn btn-secondary">
            🔄 Actualiser
          </button>
          <button (click)="exportTransactions()" class="btn btn-primary">
            📄 Exporter CSV
          </button>
        </div>
      </div>

      <div class="filters">
        <input type="text" [(ngModel)]="searchTerm" (input)="filterTransactions()"
               placeholder="Rechercher..." class="search-input">
        <select [(ngModel)]="filterType" (change)="filterTransactions()" class="filter-select">
          <option value="">Tous les types</option>
          <option value="income">Revenus</option>
          <option value="expense">Dépenses</option>
        </select>
      </div>

      <div class="transaction-table">
        <table>
          <thead>
            <tr>
              <th>Date</th>
              <th>Description</th>
              <th>Catégorie</th>
              <th>Type</th>
              <th>Montant</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            <tr *ngFor="let transaction of filteredTransactions" 
                [class.income]="transaction.type === 'income'"
                [class.expense]="transaction.type === 'expense'">
              <td>{{ transaction.date | date:'dd/MM/yyyy' }}</td>
              <td>{{ transaction.description }}</td>
              <td>{{ transaction.category }}</td>
              <td>
                <span class="type-badge" [class]="transaction.type">
                  {{ transaction.type === 'income' ? 'Revenu' : 'Dépense' }}
                </span>
              </td>
              <td class="amount">
                {{ transaction.amount | currency:'EUR':'symbol':'1.2-2' }}
              </td>
              <td class="actions">
                <button (click)="editTransaction(transaction)" 
                        class="btn btn-sm btn-primary">✏️</button>
                <button (click)="deleteTransaction(transaction.id!)" 
                        class="btn btn-sm btn-danger">🗑️</button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <div class="summary">
        <div class="summary-item income">
          <strong>Total Revenus: {{ totalIncome | currency:'EUR':'symbol':'1.2-2' }}</strong>
        </div>
        <div class="summary-item expense">
          <strong>Total Dépenses: {{ totalExpenses | currency:'EUR':'symbol':'1.2-2' }}</strong>
        </div>
        <div class="summary-item balance" [class.positive]="balance >= 0" [class.negative]="balance < 0">
          <strong>Solde: {{ balance | currency:'EUR':'symbol':'1.2-2' }}</strong>
        </div>
      </div>
    </div>
  `,
  styles: [`
    .transaction-list {
      padding: 20px;
    }
    .header {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 20px;
    }
    .actions, .filters {
      display: flex;
      gap: 10px;
      margin-bottom: 20px;
    }
    .search-input, .filter-select {
      padding: 8px 12px;
      border: 1px solid #ccc;
      border-radius: 4px;
    }
    table {
      width: 100%;
      border-collapse: collapse;
      margin-bottom: 20px;
    }
    th, td {
      padding: 12px;
      text-align: left;
      border-bottom: 1px solid #ddd;
    }
    th {
      background-color: #f5f5f5;
      font-weight: bold;
    }
    .income {
      background-color: rgba(40, 167, 69, 0.1);
    }
    .expense {
      background-color: rgba(220, 53, 69, 0.1);
    }
    .type-badge {
      padding: 4px 8px;
      border-radius: 12px;
      font-size: 12px;
      color: white;
    }
    .type-badge.income {
      background-color: #28a745;
    }
    .type-badge.expense {
      background-color: #dc3545;
    }
    .amount {
      text-align: right;
      font-family: monospace;
    }
    .btn {
      padding: 8px 16px;
      border: none;
      border-radius: 4px;
      cursor: pointer;
    }
    .btn-sm {
      padding: 4px 8px;
      font-size: 12px;
    }
    .summary {
      display: flex;
      justify-content: space-around;
      padding: 20px;
      background-color: #f8f9fa;
      border-radius: 8px;
    }
    .summary-item.positive {
      color: #28a745;
    }
    .summary-item.negative {
      color: #dc3545;
    }
  `]
})
export class TransactionListComponent implements OnInit {
  transactions: Transaction[] = [];
  filteredTransactions: Transaction[] = [];
  searchTerm = '';
  filterType = '';

  get totalIncome(): number {
    return this.transactions
      .filter(t => t.type === 'income')
      .reduce((sum, t) => sum + t.amount, 0);
  }

  get totalExpenses(): number {
    return this.transactions
      .filter(t => t.type === 'expense')
      .reduce((sum, t) => sum + t.amount, 0);
  }

  get balance(): number {
    return this.totalIncome - this.totalExpenses;
  }

  constructor(private tauriService: TauriService) {}

  async ngOnInit() {
    await this.refreshTransactions();
  }

  async refreshTransactions() {
    try {
      this.transactions = await this.tauriService.getTransactions(500);
      this.filterTransactions();
    } catch (error) {
      alert('Erreur lors du chargement des transactions : ' + error);
    }
  }

  filterTransactions() {
    this.filteredTransactions = this.transactions.filter(transaction => {
      const matchesSearch = transaction.description.toLowerCase()
        .includes(this.searchTerm.toLowerCase()) ||
        transaction.category.toLowerCase()
          .includes(this.searchTerm.toLowerCase());
      
      const matchesType = !this.filterType || transaction.type === this.filterType;
      
      return matchesSearch && matchesType;
    });
  }

  async deleteTransaction(id: number) {
    if (confirm('Êtes-vous sûr de vouloir supprimer cette transaction ?')) {
      try {
        await this.tauriService.deleteTransaction(id);
        await this.refreshTransactions();
        alert('Transaction supprimée avec succès !');
      } catch (error) {
        alert('Erreur lors de la suppression : ' + error);
      }
    }
  }

  editTransaction(transaction: Transaction) {
    // Émettre un événement pour notifier le composant parent
    // Ou naviguer vers un composant d'édition
    console.log('Édition de la transaction:', transaction);
  }

  async exportTransactions() {
    try {
      const csvData = await this.tauriService.exportData('csv');
      // Créer un fichier et le télécharger
      const blob = new Blob([csvData], { type: 'text/csv' });
      const url = window.URL.createObjectURL(blob);
      const link = document.createElement('a');
      link.href = url;
      link.download = `transactions_${new Date().toISOString().split('T')[0]}.csv`;
      link.click();
      window.URL.revokeObjectURL(url);
    } catch (error) {
      alert('Erreur lors de l\'export : ' + error);
    }
  }
}
```

### **3. Dashboard de Comptabilité**

#### **📄 src/app/accounting/components/dashboard/dashboard.component.ts**
```typescript
import { Component, OnInit } from '@angular/core';
import { TauriService } from '../../../core/services/tauri/tauri.service';

@Component({
  selector: 'app-dashboard',
  template: `
    <div class="dashboard">
      <h2>Tableau de Bord Comptable</h2>
      
      <div class="dashboard-grid">
        <!-- Résumé mensuel -->
        <div class="card">
          <h3>Résumé Mensuel</h3>
          <div class="month-selector">
            <select [(ngModel)]="selectedYear" (change)="loadMonthlyReport()">
              <option *ngFor="let year of years" [value]="year">{{ year }}</option>
            </select>
            <select [(ngModel)]="selectedMonth" (change)="loadMonthlyReport()">
              <option *ngFor="let month of months; index as i" [value]="i + 1">
                {{ month }}
              </option>
            </select>
          </div>
          
          <div class="monthly-stats" *ngIf="monthlyReport">
            <div class="stat income">
              <span class="label">Revenus:</span>
              <span class="value">{{ monthlyReport.income | currency:'EUR':'symbol':'1.2-2' }}</span>
            </div>
            <div class="stat expense">
              <span class="label">Dépenses:</span>
              <span class="value">{{ monthlyReport.expenses | currency:'EUR':'symbol':'1.2-2' }}</span>
            </div>
            <div class="stat balance" [class.positive]="monthlyReport.balance >= 0" 
                 [class.negative]="monthlyReport.balance < 0">
              <span class="label">Solde:</span>
              <span class="value">{{ monthlyReport.balance | currency:'EUR':'symbol':'1.2-2' }}</span>
            </div>
          </div>
        </div>

        <!-- Dépenses par catégorie -->
        <div class="card">
          <h3>Dépenses par Catégorie</h3>
          <div class="categories" *ngIf="monthlyReport?.categories">
            <div *ngFor="let category of getCategoriesArray()" class="category-item">
              <span class="category-name">{{ category.name }}</span>
              <span class="category-amount">{{ category.amount | currency:'EUR':'symbol':'1.2-2' }}</span>
              <div class="category-bar">
                <div class="category-fill" 
                     [style.width.%]="getCategoryPercentage(category.amount)"></div>
              </div>
            </div>
          </div>
        </div>

        <!-- Actions rapides -->
        <div class="card">
          <h3>Actions Rapides</h3>
          <div class="quick-actions">
            <button (click)="backupData()" class="action-btn backup">
              💾 Sauvegarder
            </button>
            <button (click)="exportMonthly()" class="action-btn export">
              📊 Export Mensuel
            </button>
            <button (click)="showImport = !showImport" class="action-btn import">
              📥 Importer
            </button>
          </div>
          
          <div *ngIf="showImport" class="import-section">
            <input type="file" #fileInput accept=".csv" (change)="onFileSelected($event)">
            <button (click)="importCsv()" [disabled]="!selectedFile" class="btn btn-primary">
              Importer CSV
            </button>
          </div>
        </div>

        <!-- Comptes -->
        <div class="card">
          <h3>Comptes</h3>
          <div class="accounts" *ngIf="accounts.length > 0">
            <div *ngFor="let account of accounts" class="account-item">
              <span class="account-name">{{ account.name }}</span>
              <span class="account-balance" 
                    [class.positive]="account.balance >= 0" 
                    [class.negative]="account.balance < 0">
                {{ account.balance | currency:'EUR':'symbol':'1.2-2' }}
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>
  `,
  styles: [`
    .dashboard {
      padding: 20px;
    }
    .dashboard-grid {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
      gap: 20px;
      margin-top: 20px;
    }
    .card {
      background: white;
      border-radius: 8px;
      padding: 20px;
      box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    }
    .card h3 {
      margin-top: 0;
      color: #333;
      border-bottom: 2px solid #007bff;
      padding-bottom: 10px;
    }
    .month-selector {
      display: flex;
      gap: 10px;
      margin-bottom: 20px;
    }
    .month-selector select {
      padding: 8px;
      border: 1px solid #ddd;
      border-radius: 4px;
    }
    .monthly-stats {
      display: flex;
      flex-direction: column;
      gap: 15px;
    }
    .stat {
      display: flex;
      justify-content: space-between;
      align-items: center;
      padding: 15px;
      border-radius: 8px;
      background-color: #f8f9fa;
    }
    .stat.income {
      border-left: 4px solid #28a745;
    }
    .stat.expense {
      border-left: 4px solid #dc3545;
    }
    .stat.balance.positive {
      border-left: 4px solid #28a745;
      background-color: rgba(40, 167, 69, 0.1);
    }
    .stat.balance.negative {
      border-left: 4px solid #dc3545;
      background-color: rgba(220, 53, 69, 0.1);
    }
    .categories {
      display: flex;
      flex-direction: column;
      gap: 10px;
    }
    .category-item {
      display: flex;
      flex-direction: column;
      gap: 5px;
    }
    .category-bar {
      height: 20px;
      background-color: #e9ecef;
      border-radius: 10px;
      overflow: hidden;
    }
    .category-fill {
      height: 100%;
      background-color: #007bff;
      transition: width 0.3s ease;
    }
    .quick-actions {
      display: flex;
      flex-direction: column;
      gap: 10px;
    }
    .action-btn {
      padding: 12px;
      border: none;
      border-radius: 6px;
      cursor: pointer;
      font-size: 14px;
      transition: background-color 0.3s;
    }
    .action-btn.backup {
      background-color: #6c757d;
      color: white;
    }
    .action-btn.export {
      background-color: #17a2b8;
      color: white;
    }
    .action-btn.import {
      background-color: #28a745;
      color: white;
    }
    .import-section {
      margin-top: 15px;
      padding-top: 15px;
      border-top: 1px solid #dee2e6;
    }
    .accounts {
      display: flex;
      flex-direction: column;
      gap: 10px;
    }
    .account-item {
      display: flex;
      justify-content: space-between;
      align-items: center;
      padding: 10px;
      background-color: #f8f9fa;
      border-radius: 6px;
    }
    .positive {
      color: #28a745;
    }
    .negative {
      color: #dc3545;
    }
  `]
})
export class DashboardComponent implements OnInit {
  monthlyReport: any = null;
  accounts: any[] = [];
  selectedYear = new Date().getFullYear();
  selectedMonth = new Date().getMonth() + 1;
  showImport = false;
  selectedFile: File | null = null;

  years = Array.from({ length: 10 }, (_, i) => new Date().getFullYear() - i);
  months = [
    'Janvier', 'Février', 'Mars', 'Avril', 'Mai', 'Juin',
    'Juillet', 'Août', 'Septembre', 'Octobre', 'Novembre', 'Décembre'
  ];

  constructor(private tauriService: TauriService) {}

  async ngOnInit() {
    await this.loadMonthlyReport();
    await this.loadAccounts();
  }

  async loadMonthlyReport() {
    try {
      this.monthlyReport = await this.tauriService.getMonthlyReport(
        this.selectedYear, 
        this.selectedMonth
      );
    } catch (error) {
      console.error('Erreur chargement rapport mensuel:', error);
    }
  }

  async loadAccounts() {
    try {
      this.accounts = await this.tauriService.getAccounts();
    } catch (error) {
      console.error('Erreur chargement comptes:', error);
    }
  }

  getCategoriesArray(): any[] {
    if (!this.monthlyReport?.categories) return [];
    
    return Object.entries(this.monthlyReport.categories)
      .map(([name, amount]) => ({ name, amount }))
      .sort((a: any, b: any) => b.amount - a.amount);
  }

  getCategoryPercentage(amount: number): number {
    if (!this.monthlyReport?.expenses || this.monthlyReport.expenses === 0) return 0;
    return (amount / this.monthlyReport.expenses) * 100;
  }

  async backupData() {
    try {
      const backupPath = await this.tauriService.backupDatabase();
      alert(`Sauvegarde créée : ${backupPath}`);
    } catch (error) {
      alert('Erreur lors de la sauvegarde : ' + error);
    }
  }

  async exportMonthly() {
    try {
      const csvData = await this.tauriService.exportData('csv');
      const blob = new Blob([csvData], { type: 'text/csv' });
      const url = window.URL.createObjectURL(blob);
      const link = document.createElement('a');
      link.href = url;
      link.download = `rapport_${this.selectedYear}_${this.selectedMonth.toString().padStart(2, '0')}.csv`;
      link.click();
      window.URL.revokeObjectURL(url);
    } catch (error) {
      alert('Erreur lors de l\'export : ' + error);
    }
  }

  onFileSelected(event: any) {
    this.selectedFile = event.target.files[0];
  }

  async importCsv() {
    if (!this.selectedFile) return;
    
    try {
      // Pour l'import, il faudrait d'abord sauvegarder le fichier localement
      // puis appeler la fonction Tauri avec le chemin du fichier
      alert('Fonctionnalité d\'import à implémenter selon vos besoins');
    } catch (error) {
      alert('Erreur lors de l\'import : ' + error);
    }
  }
}
```

---

## ⚙️ **Configuration et Permissions**

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
    "productName": "Comptabilité Pro",
    "version": "1.0.0"
  },
  "tauri": {
    "allowlist": {
      "all": false,
      "fs": {
        "all": false,
        "readFile": true,
        "writeFile": true,
        "readDir": true,
        "copyFile": true,
        "createDir": true,
        "removeDir": true,
        "removeFile": true,
        "renameFile": true,
        "exists": true,
        "scope": [
          "$APPDATA/*",
          "$DESKTOP/*",
          "$DOCUMENT/*",
          "$DOWNLOAD/*"
        ]
      },
      "dialog": {
        "all": false,
        "open": true,
        "save": true
      },
      "notification": {
        "all": true
      },
      "path": {
        "all": true
      },
      "window": {
        "all": false,
        "close": true,
        "hide": true,
        "show": true,
        "maximize": true,
        "minimize": true,
        "unmaximize": true,
        "unminimize": true,
        "startDragging": true
      }
    },
    "bundle": {
      "active": true,
      "category": "Finance",
      "copyright": "",
      "deb": {
        "depends": []
      },
      "externalBin": [],
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/128x128@2x.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ],
      "identifier": "com.accounting.app",
      "longDescription": "Application de comptabilité personnelle avec base de données locale",
      "macOS": {
        "entitlements": null,
        "exceptionDomain": "",
        "frameworks": [],
        "providerShortName": null,
        "signingIdentity": null
      },
      "resources": [],
      "shortDescription": "Comptabilité Pro",
      "targets": "all",
      "windows": {
        "certificateThumbprint": null,
        "digestAlgorithm": "sha256",
        "timestampUrl": ""
      }
    },
    "security": {
      "csp": null
    },
    "updater": {
      "active": false
    },
    "windows": [
      {
        "fullscreen": false,
        "height": 800,
        "resizable": true,
        "title": "Comptabilité Pro",
        "width": 1200,
        "minWidth": 800,
        "minHeight": 600
      }
    ]
  }
}
```

### **📄 src-tauri/Cargo.toml**
```toml
[package]
name = "angular-tauri"
version = "1.0.0"
description = "Application de comptabilité avec Angular et Tauri"
authors = ["Votre Nom <email@example.com>"]
license = ""
repository = ""
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[build-dependencies]
tauri-build = { version = "1.5", features = [] }

[dependencies]
tauri = { version = "1.7", features = ["api-all"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
rusqlite = { version = "0.29", features = ["bundled"] }
chrono = { version = "0.4", features = ["serde"] }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }

[features]
# This feature is used for production builds or when `devPath` points to the filesystem
# DO NOT REMOVE!!
custom-protocol = ["tauri/custom-protocol"]
```

---

## 🚀 **Démarrage et Build**

### **Scripts Package.json**
```json
{
  "scripts": {
    "ng": "ng",
    "start": "ng serve",
    "build": "ng build",
    "watch": "ng build --watch --configuration development",
    "test": "ng test",
    "tauri": "tauri",
    "tauri:dev": "tauri dev",
    "tauri:build": "tauri build",
    "tauri:icon": "tauri icon"
  }
}
```

### **Commandes de Développement**

```bash
# Installer les dépendances
npm install

# Développement (Frontend + Backend avec hot reload)
npm run tauri:dev

# Build de production
npm run tauri:build

# Tests Frontend
npm run test

# Générer des icônes
npm run tauri:icon assets/icon.png
```

---

## � **Sécurité et Bonnes Pratiques**

### **1. Validation des Données**
- Toujours valider les entrées côté Frontend ET Backend
- Utiliser des types TypeScript stricts
- Implémenter des contraintes de base de données

### **2. Gestion des Erreurs**
```typescript
// Frontend
async createTransaction(transaction: Transaction) {
  try {
    const id = await this.tauriService.createTransaction(transaction);
    this.notificationService.success('Transaction créée !');
    return id;
  } catch (error) {
    this.notificationService.error('Erreur: ' + error);
    throw error;
  }
}
```

```rust
// Backend
#[command]
pub async fn create_transaction(transaction: Transaction) -> Result<i64, String> {
    // Validation
    if transaction.amount <= 0.0 {
        return Err("Le montant doit être positif".to_string());
    }
    
    if transaction.description.trim().is_empty() {
        return Err("La description est obligatoire".to_string());
    }
    
    // ... logique de création
}
```

### **3. Permissions Minimales**
- Ne donner accès qu'aux APIs nécessaires
- Limiter les scopes de fichiers
- Utiliser HTTPS pour les communications externes

---

## 📈 **Fonctionnalités Avancées**

### **1. Import/Export Automatique**
- Import bancaire CSV/OFX
- Synchronisation cloud (optionnelle)
- Rapports PDF automatiques

### **2. Analyses Avancées**
- Graphiques de tendances
- Prévisions budgétaires
- Alertes de dépassement

### **3. Multi-utilisateurs**
- Comptes séparés
- Partage de données familiales
- Contrôle d'accès

---

Ce tutoriel vous donne une base solide pour créer une application de comptabilité complète avec Angular et Tauri, incluant une base de données SQLite locale et toutes les fonctionnalités essentielles ! 🚀