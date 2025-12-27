import { Injectable } from '@angular/core';
import { invoke } from '@tauri-apps/api/core';
import { NumberData, NumberStats } from '../../core/services/tauri/tauri.interfaces';

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