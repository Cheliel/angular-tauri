# Erreur PowerShell : Script Non Signé

## Résumé du Problème

### Erreur Rencontrée
```
npm : Impossible de charger le fichier C:\Program Files\nodejs\npm.ps1. 
Le fichier C:\Program Files\nodejs\npm.ps1 n'est pas signé numériquement. 
Vous ne pouvez pas exécuter ce script sur le système actuel.
```

### Pourquoi Cette Erreur Apparaît-elle ?

Cette erreur survient à cause de la **politique d'exécution PowerShell** de Windows. Par défaut, Windows bloque l'exécution de scripts PowerShell non signés numériquement pour des raisons de sécurité.

**Causes principales :**
- La politique d'exécution est configurée sur `Restricted` ou `AllSigned`
- Les scripts npm/node utilisent PowerShell sur Windows
- Ces scripts ne sont pas signés numériquement par Microsoft
- Windows refuse d'exécuter des scripts non approuvés

---

## Solutions

### ✅ Solution Temporaire (Recommandée)

Cette solution affecte uniquement la session PowerShell actuelle :

```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope Process
```

**Avantages :**
- ✅ Sécuritaire (n'affecte que la session courante)
- ✅ Temporaire (se réinitialise à la fermeture du terminal)
- ✅ Permet l'exécution des commandes npm/ng

**Inconvénients :**
- ❌ À refaire à chaque nouvelle session PowerShell

---

### 🔧 Solution Permanente (Pour Utilisateur Courant)

⚠️ **Attention : Impact sur la sécurité**

Pour l'utilisateur courant uniquement :

```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

**Vérification :**
```powershell
Get-ExecutionPolicy -List
```

**Avantages :**
- ✅ Permanent pour l'utilisateur courant
- ✅ N'affecte pas les autres utilisateurs du système
- ✅ Plus pratique pour le développement

**Inconvénients :**
- ❌ Réduit légèrement la sécurité
- ❌ Scripts distants non signés peuvent s'exécuter

---

### 🛡️ Solution Permanente (Système Complet)

⚠️ **DANGER : Impact sécurité élevé - Non recommandé**

Pour tout le système (nécessite des privilèges administrateur) :

```powershell
# Exécuter PowerShell en tant qu'Administrateur
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope LocalMachine
```

**⚠️ Risques :**
- Affecte tous les utilisateurs du système
- Réduit significativement la sécurité
- Peut permettre l'exécution de scripts malveillants

---

## Comprendre les Politiques d'Exécution

### Types de Politiques

| Politique | Description | Sécurité |
|-----------|-------------|----------|
| `Restricted` | Aucun script autorisé | 🔒 Maximale |
| `AllSigned` | Seuls les scripts signés | 🔒 Haute |
| `RemoteSigned` | Scripts locaux + signés distants | ⚖️ Équilibrée |
| `Unrestricted` | Tous les scripts | ⚠️ Faible |
| `Bypass` | Aucune restriction | ❌ Aucune |
| `Undefined` | **Hérite du niveau supérieur** | 📋 *Variable* |

### Hiérarchie des Politiques (Ordre de Priorité)
1. **MachinePolicy** (Politique domaine/groupe) - *Priorité maximale*
2. **UserPolicy** (Politique utilisateur domaine)
3. **Process** (Session PowerShell courante)
4. **CurrentUser** (Utilisateur local)  
5. **LocalMachine** (Système complet) - *Priorité minimale*

> **Règle :** La première politique **non-Undefined** dans l'ordre de priorité est appliquée.

### 🔍 Comprendre "Undefined"

**`Undefined` n'est PAS une politique de sécurité** mais un **état d'héritage**.

#### Exemple Concret de Votre Configuration :
```powershell
MachinePolicy: Undefined  → Passe au suivant
UserPolicy:    Undefined  → Passe au suivant  
Process:       RemoteSigned ← ACTIF (solution temporaire)
CurrentUser:   Undefined  → Ignoré (Process a priorité)
LocalMachine:  AllSigned  → Serait actif sans Process
```

**Résultat :** PowerShell utilise `RemoteSigned` (scope Process)

#### ⚠️ Après Fermeture de PowerShell :
```powershell
Process:       Undefined  ← Plus de politique temporaire
CurrentUser:   Undefined  → Passe au suivant
LocalMachine:  AllSigned  ← REDEVIENT ACTIF !
```

**Résultat :** Retour au problème (`AllSigned` trop restrictif)

#### ✅ Solution Permanente :
```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

Résultat final :
```powershell
CurrentUser:   RemoteSigned ← NOUVELLE POLITIQUE PERMANENTE
LocalMachine:  AllSigned    ← Ignoré (CurrentUser prioritaire)
```

### Vérifier la Politique Actuelle

```powershell
Get-ExecutionPolicy
Get-ExecutionPolicy -List
```

---

## Workflow Recommandé pour le Développement

### 1. Méthode Sécurisée (Recommandée)
```powershell
# À chaque nouvelle session PowerShell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope Process

# Puis utiliser npm/ng normalement
npm install
ng add @ng-bootstrap/ng-bootstrap
```

### 2. Méthode Pratique (Acceptable)
```powershell
# Une seule fois par utilisateur
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

### 3. Alternative : Utiliser CMD
Si vous préférez éviter de modifier la politique :
```cmd
# Utiliser l'invite de commandes classique
cmd
npm install
```

---

## Cas d'Utilisation Spécifiques

### Pour Angular CLI
```powershell
# Résoudre temporairement
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope Process

# Installer Angular CLI
npm install -g @angular/cli

# Utiliser ng commands
ng add @ng-bootstrap/ng-bootstrap
```

### Pour npm Scripts
```powershell
# Résoudre temporairement
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope Process

# Exécuter des scripts npm
npm run build
npm run dev
npm start
```

---

## Dépannage

### Erreur Persiste Après Solution Temporaire ?
- Fermer et rouvrir PowerShell
- Vérifier avec `Get-ExecutionPolicy`
- Réessayer la commande `Set-ExecutionPolicy`

### Problèmes avec Visual Studio Code ?
- Fermer VS Code complètement
- Rouvrir VS Code
- Les terminaux intégrés hériteront de la nouvelle politique

### Alternative : Utiliser PowerShell 7
PowerShell 7 (Core) a des politiques différentes et peut résoudre certains problèmes.

---

## Sécurité et Bonnes Pratiques

### ✅ Recommandations
- Utiliser la solution temporaire (`-Scope Process`)
- Revenir à `Restricted` après développement si nécessaire
- Éviter `Unrestricted` ou `Bypass`
- Vérifier périodiquement les politiques actives

### ❌ À Éviter
- Politique `Unrestricted` en permanence
- Modification système-wide sans nécessité
- Ignorer les avertissements de sécurité

---

## Pourquoi ce Problème Apparaît Soudainement ?

Si vous n'avez jamais eu ce problème avant sur le même ordinateur, voici les causes les plus probables :

### 🔄 Changements Récents Système
- **Mise à jour Windows** : Les mises à jour de sécurité peuvent réinitialiser les politiques d'exécution de `RemoteSigned` vers `AllSigned`
- **Politique d'entreprise** : Nouvelle GPO (Group Policy Object) appliquée
- **Mise à jour antivirus** : Certains antivirus modifient les politiques PowerShell
- **Windows Defender** : Changement des paramètres de sécurité

### 📊 Vérifier l'Historique des Changements
```powershell
# Vérifier les mises à jour récentes
Get-WinEvent -FilterHashtable @{LogName='System'; ID=19} -MaxEvents 10

# Voir toutes les politiques actuelles
Get-ExecutionPolicy -List
```

### 🔧 Restaurer l'Ancienne Configuration
Si ça marchait avant, restaurez avec :
```powershell
# Solution permanente (utilisateur courant)
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

---

*Ce guide couvre les solutions pour Windows PowerShell 5.1 et PowerShell 7. Pour d'autres shells (CMD, Git Bash), ces problèmes ne se posent généralement pas.*