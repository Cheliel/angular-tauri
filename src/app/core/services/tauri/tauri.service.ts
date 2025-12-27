import { Injectable } from '@angular/core';
import { invoke } from '@tauri-apps/api/core'
import { NumberData } from '../tauri/tauri.interfaces';


@Injectable({
  providedIn: 'root'
})
export class TauriService {

  constructor() {
  }

  get isTauri(): boolean {
    return !!(window && window.__TAURI__);
  }

  async callHelloWorld() {
    const text = await invoke('hello_world_command');
    console.log(text);
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

async getNumbersStats(): Promise<any> {
  try {
    return await invoke('get_numbers_stats');
  } catch (error) {
    console.error('Erreur stats nombres:', error);
    throw error;
  }
}
}
