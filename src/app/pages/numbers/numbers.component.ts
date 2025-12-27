import { Component, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { RouterLink } from '@angular/router';
import { TauriService } from '../../core/services/tauri/tauri.service';

@Component({
  selector: 'app-numbers',
  templateUrl: './numbers.component.html',
  styleUrls: ['./numbers.component.scss'],
  standalone: true,
  imports: [CommonModule, FormsModule, RouterLink]
})
export class NumbersComponent implements OnInit {
  
  numbers: number[] = [];
  newNumber: string = '';
  loading = false;
  message = '';

  constructor(private tauriService: TauriService) {}

  async ngOnInit() {
    await this.loadNumbers();
  }

  // Charger les nombres depuis Tauri
  async loadNumbers() {
    this.loading = true;
    try {
      if (this.tauriService.isTauri) {
        this.numbers = await this.tauriService.loadNumbers();
        this.showMessage('Données chargées avec succès', 'success');
      } else {
        // Données de test pour le navigateur
        this.numbers = [1, 2, 3, 42, 100];
        this.showMessage('Mode navigateur - données de test', 'info');
      }
    } catch (error) {
      console.error('Erreur chargement:', error);
      this.showMessage('Erreur lors du chargement', 'error');
    } finally {
      this.loading = false;
    }
  }

  // Ajouter un nouveau nombre
  async addNumber() {
    const num = parseFloat(this.newNumber.trim());
    
    // Validation
    if (isNaN(num)) {
      this.showMessage('Veuillez entrer un nombre valide', 'error');
      return;
    }

    if (this.numbers.includes(num)) {
      this.showMessage('Ce nombre existe déjà dans le tableau', 'warning');
      return;
    }

    // Ajouter le nombre
    this.numbers.push(num);
    this.newNumber = '';

    // Sauvegarder avec Tauri
    await this.saveNumbers();
    this.showMessage(`Nombre ${num} ajouté avec succès`, 'success');
  }

  // Supprimer un nombre
  async removeNumber(index: number) {
    const removedNumber = this.numbers[index];
    this.numbers.splice(index, 1);
    
    await this.saveNumbers();
    this.showMessage(`Nombre ${removedNumber} supprimé`, 'info');
  }

  // Sauvegarder avec Tauri
  async saveNumbers() {
    if (this.tauriService.isTauri) {
      try {
        await this.tauriService.saveNumbers(this.numbers);
      } catch (error) {
        console.error('Erreur sauvegarde:', error);
        this.showMessage('Erreur lors de la sauvegarde', 'error');
      }
    }
  }

  // Effacer tout le tableau
  async clearAll() {
    if (confirm('Êtes-vous sûr de vouloir effacer tous les nombres ?')) {
      this.numbers = [];
      await this.saveNumbers();
      this.showMessage('Tableau effacé', 'info');
    }
  }

  // Calculer des statistiques simples
  get statistics() {
    if (this.numbers.length === 0) {
      return { count: 0, sum: 0, average: 0, min: 0, max: 0 };
    }

    const sum = this.numbers.reduce((a, b) => a + b, 0);
    const average = sum / this.numbers.length;
    const min = Math.min(...this.numbers);
    const max = Math.max(...this.numbers);

    return {
      count: this.numbers.length,
      sum: sum,
      average: Math.round(average * 100) / 100,
      min: min,
      max: max
    };
  }

  // Trier le tableau
  async sortNumbers(order: 'asc' | 'desc') {
    this.numbers.sort((a, b) => order === 'asc' ? a - b : b - a);
    await this.saveNumbers();
    this.showMessage(`Tableau trié (${order === 'asc' ? 'croissant' : 'décroissant'})`, 'info');
  }

  // Afficher un message temporaire
  private showMessage(text: string, type: 'success' | 'error' | 'warning' | 'info') {
    this.message = text;
    setTimeout(() => {
      this.message = '';
    }, 3000);
  }

  // Gérer la touche Entrée
  onKeyPress(event: KeyboardEvent) {
    if (event.key === 'Enter') {
      this.addNumber();
    }
  }
}