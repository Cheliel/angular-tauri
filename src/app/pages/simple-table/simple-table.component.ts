import { Component, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { RouterLink } from '@angular/router';
import { TauriService } from '../../core/services/tauri/tauri.service';

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