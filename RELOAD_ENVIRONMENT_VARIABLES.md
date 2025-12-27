# Recharger les Variables d'Environnement Manuellement

## 🎯 **Problème Résolu**

Après l'installation de Rust/Cargo, les variables d'environnement ne sont pas immédiatement disponibles dans le terminal actuel. Ce guide explique comment les recharger sans redémarrer.

---

## 🔄 **Méthodes de Rechargement**

### **1. Recharger Variables PowerShell (Méthode Simple)**

#### **Commande Directe**
```powershell
# Recharger le PATH depuis le registre
$env:PATH = [System.Environment]::GetEnvironmentVariable("PATH", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("PATH", "User")
```

#### **Verification**
```powershell
# Tester si Cargo est disponible
cargo --version
rustc --version
```

---

### **2. Script Complet de Rechargement**

#### **Créer et Exécuter le Script**
```powershell
# Créer un script de rechargement
@"
# Recharger toutes les variables d'environnement
foreach(`$level in "Machine","User") {
    [Environment]::GetEnvironmentVariables(`$level).GetEnumerator() | % {
        # Pour PATH, on ajoute au lieu d'écraser
        if(`$_.Name -match 'Path') {
            `$existing = [Environment]::GetEnvironmentVariable(`$_.Name, "Process")
            if(`$existing -notlike "*`$(`$_.Value)*") {
                [Environment]::SetEnvironmentVariable(`$_.Name, "`$existing;`$(`$_.Value)", "Process")
            }
        } else {
            [Environment]::SetEnvironmentVariable(`$_.Name, `$_.Value, "Process")
        }
    }
}

Write-Host "Variables d'environnement rechargées!" -ForegroundColor Green
"@ | Out-File -FilePath "reload-env.ps1" -Encoding UTF8

# Exécuter le script
.\reload-env.ps1

# Nettoyer le script temporaire
Remove-Item "reload-env.ps1"
```

---

### **3. Méthode avec Chocolatey (Si installé)**

#### **Import du Module Chocolatey**
```powershell
# Si vous avez Chocolatey installé
Import-Module $env:ChocolateyInstall\helpers\chocolateyProfile.psm1

# Utiliser la fonction refreshenv
refreshenv
```

---

### **4. Rechargement Spécifique Rust/Cargo**

#### **Ajouter Manuellement le Chemin Cargo**
```powershell
# Chemin standard de Cargo sur Windows
$cargoPath = "$env:USERPROFILE\.cargo\bin"

# Vérifier si le chemin existe
if (Test-Path $cargoPath) {
    # Ajouter au PATH de la session actuelle
    if ($env:PATH -notlike "*$cargoPath*") {
        $env:PATH += ";$cargoPath"
        Write-Host "Cargo ajouté au PATH: $cargoPath" -ForegroundColor Green
    } else {
        Write-Host "Cargo déjà dans le PATH" -ForegroundColor Yellow
    }
} else {
    Write-Host "Chemin Cargo introuvable: $cargoPath" -ForegroundColor Red
}

# Vérifier la disponibilité
cargo --version
```

---

### **5. Méthode Automatique (Fonction PowerShell)**

#### **Créer une Fonction Réutilisable**
```powershell
# Ajouter à votre profil PowerShell pour usage permanent
function Refresh-Environment {
    [CmdletBinding()]
    param()
    
    Write-Host "Rechargement des variables d'environnement..." -ForegroundColor Cyan
    
    # Variables Machine
    $machineEnv = [Environment]::GetEnvironmentVariables([EnvironmentVariableTarget]::Machine)
    
    # Variables User
    $userEnv = [Environment]::GetEnvironmentVariables([EnvironmentVariableTarget]::User)
    
    # Fusionner et appliquer
    $allEnv = @{}
    $machineEnv.GetEnumerator() | ForEach-Object { $allEnv[$_.Key] = $_.Value }
    $userEnv.GetEnumerator() | ForEach-Object { $allEnv[$_.Key] = $_.Value }
    
    # Appliquer au processus courant
    foreach ($kvp in $allEnv.GetEnumerator()) {
        [Environment]::SetEnvironmentVariable($kvp.Key, $kvp.Value, [EnvironmentVariableTarget]::Process)
    }
    
    # Construire un nouveau PATH combiné
    $machinePath = [Environment]::GetEnvironmentVariable("PATH", [EnvironmentVariableTarget]::Machine)
    $userPath = [Environment]::GetEnvironmentVariable("PATH", [EnvironmentVariableTarget]::User)
    $newPath = $machinePath + ";" + $userPath
    
    [Environment]::SetEnvironmentVariable("PATH", $newPath, [EnvironmentVariableTarget]::Process)
    
    Write-Host "✅ Variables d'environnement rechargées!" -ForegroundColor Green
    
    # Test Rust/Cargo
    try {
        $cargoVersion = & cargo --version 2>$null
        if ($cargoVersion) {
            Write-Host "✅ Cargo disponible: $cargoVersion" -ForegroundColor Green
        }
    }
    catch {
        Write-Host "❌ Cargo non disponible après rechargement" -ForegroundColor Red
    }
}

# Utiliser la fonction
Refresh-Environment
```

---

## 🛠️ **Diagnostic et Dépannage**

### **Vérifier l'Installation Rust**

#### **Localisation de Rust**
```powershell
# Chercher les exécutables Rust
Get-Command cargo -ErrorAction SilentlyContinue
Get-Command rustc -ErrorAction SilentlyContinue

# Chercher dans les emplacements standards
$commonPaths = @(
    "$env:USERPROFILE\.cargo\bin",
    "$env:PROGRAMFILES\Rust stable MSVC 1.70\bin",
    "C:\Program Files\Rust stable MSVC 1.70\bin"
)

foreach ($path in $commonPaths) {
    if (Test-Path "$path\cargo.exe") {
        Write-Host "✅ Cargo trouvé dans: $path" -ForegroundColor Green
        Write-Host "Version: $(& "$path\cargo.exe" --version)" -ForegroundColor Cyan
    }
}
```

#### **Vérifier le Registry**
```powershell
# Vérifier les variables dans le registre
Get-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\Session Manager\Environment" -Name PATH
Get-ItemProperty -Path "HKCU:\Environment" -Name PATH -ErrorAction SilentlyContinue
```

### **Résoudre les Problèmes Courants**

#### **PATH Corrompu ou Trop Long**
```powershell
# Nettoyer et reconstruire le PATH
$machinePath = [Environment]::GetEnvironmentVariable("PATH", "Machine")
$userPath = [Environment]::GetEnvironmentVariable("PATH", "User")

# Supprimer les doublons et chemins vides
$cleanMachinePath = ($machinePath -split ';' | Where-Object { $_ -and (Test-Path $_ -ErrorAction SilentlyContinue) }) -join ';'
$cleanUserPath = ($userPath -split ';' | Where-Object { $_ -and (Test-Path $_ -ErrorAction SilentlyContinue) }) -join ';'

# Appliquer le PATH nettoyé
$env:PATH = "$cleanMachinePath;$cleanUserPath"
```

#### **Réinstallation Rust si Nécessaire**
```powershell
# Désinstaller Rust avec Winget
winget uninstall Rustlang.Rust.MSVC

# Réinstaller Rust
winget install Rustlang.Rust.MSVC

# Ou utiliser rustup (recommandé)
# Télécharger depuis https://rustup.rs/ et installer
```

---

## 📋 **Procédure Complète Recommandée**

### **Étapes dans l'Ordre**

```powershell
# 1. Diagnostic initial
Write-Host "=== DIAGNOSTIC INITIAL ===" -ForegroundColor Yellow
Get-Command cargo -ErrorAction SilentlyContinue
$env:PATH -split ';' | Where-Object { $_ -like "*cargo*" -or $_ -like "*rust*" }

# 2. Rechargement des variables
Write-Host "=== RECHARGEMENT VARIABLES ===" -ForegroundColor Yellow
$env:PATH = [System.Environment]::GetEnvironmentVariable("PATH", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("PATH", "User")

# 3. Ajout manuel si nécessaire
Write-Host "=== AJOUT MANUEL CARGO ===" -ForegroundColor Yellow
$cargoPath = "$env:USERPROFILE\.cargo\bin"
if ((Test-Path $cargoPath) -and ($env:PATH -notlike "*$cargoPath*")) {
    $env:PATH += ";$cargoPath"
    Write-Host "Cargo ajouté au PATH" -ForegroundColor Green
}

# 4. Vérification finale
Write-Host "=== VÉRIFICATION FINALE ===" -ForegroundColor Yellow
try {
    cargo --version
    rustc --version
    Write-Host "✅ Rust/Cargo fonctionnels!" -ForegroundColor Green
}
catch {
    Write-Host "❌ Problème persistant - Redémarrage recommandé" -ForegroundColor Red
}
```

---

## 🚀 **Après le Rechargement**

### **Test de Fonctionnement**
```powershell
# Tester Tauri
cd "C:\votre\projet\angular-tauri"
npm run tauri:dev
```

### **Si Ça Ne Marche Toujours Pas**
1. **Fermer VS Code complètement** et le rouvrir
2. **Ouvrir un nouveau terminal** Windows PowerShell  
3. **Redémarrer l'ordinateur** en dernier recours
4. **Réinstaller Rust** si le problème persiste

---

## 💡 **Conseils et Astuces**

### **Ajouter au Profil PowerShell**
Pour éviter ce problème à l'avenir, ajoutez à votre profil PowerShell :

```powershell
# Vérifier si le profil existe
if (!(Test-Path $PROFILE)) {
    New-Item -Path $PROFILE -Type File -Force
}

# Ajouter la fonction de rechargement au profil
Add-Content $PROFILE @"

# Fonction de rechargement des variables d'environnement
function Refresh-Environment {
    `$env:PATH = [System.Environment]::GetEnvironmentVariable("PATH", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("PATH", "User")
    Write-Host "Variables d'environnement rechargées!" -ForegroundColor Green
}

# Alias pour plus de facilité
Set-Alias refreshenv Refresh-Environment
"@
```

### **Variables Importantes pour le Développement**
```powershell
# Variables spécifiques souvent nécessaires
$importantVars = @("PATH", "CARGO_HOME", "RUSTUP_HOME", "RUST_SRC_PATH")
foreach ($var in $importantVars) {
    $machine = [Environment]::GetEnvironmentVariable($var, "Machine")
    $user = [Environment]::GetEnvironmentVariable($var, "User")
    if ($machine -or $user) {
        [Environment]::SetEnvironmentVariable($var, "$machine;$user", "Process")
        Write-Host "Rechargé: $var" -ForegroundColor Cyan
    }
}
```

---

*Ce guide couvre toutes les méthodes pour recharger les variables d'environnement sans redémarrage. La méthode simple fonctionne dans 90% des cas !*