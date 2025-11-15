#!/usr/bin/env pwsh

# Script para construir la aplicacion Tauri en modo produccion para 32 bits
# Este script verifica que la configuracion segura funcione correctamente

Write-Host "Construyendo aplicacion Tauri para 32 bits..." -ForegroundColor Blue

# Cambiar al directorio del proyecto
Set-Location "e:\repos\Toscanini-IngSoft"

try {
    # Verificar que el target de 32 bits este instalado
    Write-Host "Verificando target de Rust para 32 bits..." -ForegroundColor Cyan
    $targetInstalled = rustup target list --installed | Select-String "i686-pc-windows-msvc"
    if (-not $targetInstalled) {
        Write-Host "Target i686-pc-windows-msvc no esta instalado. Instalando..." -ForegroundColor Yellow
        rustup target add i686-pc-windows-msvc
        if ($LASTEXITCODE -ne 0) {
            Write-Host "Error al instalar el target. Por favor instalalo manualmente con: rustup target add i686-pc-windows-msvc" -ForegroundColor Red
            exit 1
        }
        Write-Host "Target instalado correctamente" -ForegroundColor Green
    } else {
        Write-Host "Target i686-pc-windows-msvc ya esta instalado" -ForegroundColor Green
    }
    
    # Verificar configuracion de WebView2
    Write-Host "Verificando configuracion de WebView2..." -ForegroundColor Cyan
    $tauriConfig = Get-Content "src-tauri\tauri.conf.json" | ConvertFrom-Json
    $webviewMode = $tauriConfig.bundle.windows.webviewInstallMode.type
    
    if ($webviewMode -eq "fixedRuntime") {
        $webviewPath = $tauriConfig.bundle.windows.webviewInstallMode.path
        Write-Host "WebView2 configurado como fixedRuntime" -ForegroundColor Green
        Write-Host "Ruta configurada: $webviewPath" -ForegroundColor Cyan
        
        # Verificar que la ruta existe y es para x86 (32 bits)
        $fullPath = Join-Path "src-tauri" $webviewPath
        if (Test-Path $fullPath) {
            if ($webviewPath -match "x86") {
                Write-Host "  [OK] Runtime de WebView2 encontrado en: $fullPath (x86 - correcto para 32 bits)" -ForegroundColor Green
            } elseif ($webviewPath -match "x64") {
                Write-Host "  [ERROR] La ruta apunta a x64 pero estas compilando para 32 bits!" -ForegroundColor Red
                Write-Host "  Cambia la ruta a una version x86 del runtime" -ForegroundColor Red
            } else {
                Write-Host "  [ADVERTENCIA] No se puede determinar la arquitectura del runtime" -ForegroundColor Yellow
            }
        } else {
            Write-Host "  [ERROR] No se encontro el runtime de WebView2 en: $fullPath" -ForegroundColor Red
            Write-Host "  Necesitas descargar y extraer el WebView2 Fixed Version Runtime para x86" -ForegroundColor Yellow
            Write-Host "  Descarga desde: https://developer.microsoft.com/en-us/microsoft-edge/webview2/" -ForegroundColor Yellow
        }
    } elseif ($webviewMode -eq "offlineInstaller") {
        Write-Host "WebView2 configurado como offlineInstaller (incluye instalador completo sin conexion)" -ForegroundColor Green
        Write-Host "NOTA: El bundle sera mas grande (~100-150 MB adicionales) pero incluira el instalador completo de WebView2" -ForegroundColor Yellow
    } elseif ($webviewMode -eq "embedBootstrapper") {
        Write-Host "WebView2 configurado como embedBootstrapper (recomendado para 32 bits y Windows 7)" -ForegroundColor Green
        Write-Host "NOTA: El bundle sera mas grande (~100-150 MB adicionales) pero incluira el instalador de WebView2" -ForegroundColor Yellow
    } else {
        Write-Host "WebView2 configurado como: $webviewMode" -ForegroundColor Cyan
        Write-Host "NOTA: Verifica que esta configuracion sea adecuada para sistemas de 32 bits" -ForegroundColor Yellow
    }
    
    # Configurar target de 32 bits usando variable de entorno
    # Esto funciona porque Tauri respeta las variables de entorno de Cargo
    $env:CARGO_BUILD_TARGET = "i686-pc-windows-msvc"
    Write-Host "Target configurado: i686-pc-windows-msvc" -ForegroundColor Cyan
    
    # Construir la aplicacion
    Write-Host "Ejecutando build de produccion..." -ForegroundColor Yellow
    npm run tauri build

    # Verificar que el .env no este en el bundle
    # Cuando se compila para un target especifico, la ruta cambia
    $bundlePath = "src-tauri\target\i686-pc-windows-msvc\release\bundle"
    # Fallback a la ruta estandar si no existe
    if (-not (Test-Path $bundlePath)) {
        $bundlePath = "src-tauri\target\release\bundle"
    }
    if (Test-Path $bundlePath) {
        Write-Host "Bundle creado en: $bundlePath" -ForegroundColor Green
        
        # Buscar archivos .env en el bundle
        $envFiles = Get-ChildItem -Path $bundlePath -Recurse -Name "*.env" -ErrorAction SilentlyContinue
        
        if ($envFiles.Count -eq 0) {
            Write-Host "SEGURIDAD: No se encontraron archivos .env en el bundle - Excelente!" -ForegroundColor Green
        } else {
            Write-Host "ADVERTENCIA: Se encontraron archivos .env en el bundle:" -ForegroundColor Red
            $envFiles | ForEach-Object { Write-Host "   - $_" -ForegroundColor Red }
        }
        
        # Corregir nombres de archivos: Tauri genera nombres con x64 aunque compile para 32 bits
        Write-Host "Corrigiendo nombres de archivos de x64 a x32..." -ForegroundColor Cyan
        $msiFiles = Get-ChildItem -Path "$bundlePath\msi" -Filter "*.msi" -ErrorAction SilentlyContinue
        $nsisFiles = Get-ChildItem -Path "$bundlePath\nsis" -Filter "*.exe" -ErrorAction SilentlyContinue
        
        $filesRenamed = 0
        
        foreach ($file in $msiFiles) {
            if ($file.Name -match "x64") {
                $newName = $file.Name -replace "x64", "x32"
                Rename-Item -Path $file.FullName -NewName $newName -Force
                Write-Host "  [CORREGIDO] MSI: $($file.Name) -> $newName" -ForegroundColor Yellow
                $filesRenamed++
            }
        }
        
        foreach ($file in $nsisFiles) {
            if ($file.Name -match "x64") {
                $newName = $file.Name -replace "x64", "x32"
                Rename-Item -Path $file.FullName -NewName $newName -Force
                Write-Host "  [CORREGIDO] NSIS: $($file.Name) -> $newName" -ForegroundColor Yellow
                $filesRenamed++
            }
        }
        
        if ($filesRenamed -gt 0) {
            Write-Host "Se renombraron $filesRenamed archivo(s) para reflejar la arquitectura correcta (x32)" -ForegroundColor Green
        }
        
        # Verificar nombres de archivos generados (deben contener x32 para 32 bits)
        Write-Host "Verificando nombres de archivos generados..." -ForegroundColor Cyan
        $msiFiles = Get-ChildItem -Path "$bundlePath\msi" -Filter "*.msi" -ErrorAction SilentlyContinue
        $nsisFiles = Get-ChildItem -Path "$bundlePath\nsis" -Filter "*.exe" -ErrorAction SilentlyContinue
        
        $archCorrect = $true
        
        foreach ($file in $msiFiles) {
            if ($file.Name -match "x32") {
                Write-Host "  [OK] MSI: $($file.Name) - Contiene x32 (correcto para 32 bits)" -ForegroundColor Green
            } elseif ($file.Name -match "x64") {
                Write-Host "  [ERROR] MSI: $($file.Name) - Contiene x64 (incorrecto para 32 bits)" -ForegroundColor Red
                $archCorrect = $false
            } else {
                Write-Host "  [ADVERTENCIA] MSI: $($file.Name) - No contiene indicador de arquitectura" -ForegroundColor Yellow
            }
        }
        
        foreach ($file in $nsisFiles) {
            if ($file.Name -match "x32") {
                Write-Host "  [OK] NSIS: $($file.Name) - Contiene x32 (correcto para 32 bits)" -ForegroundColor Green
            } elseif ($file.Name -match "x64") {
                Write-Host "  [ERROR] NSIS: $($file.Name) - Contiene x64 (incorrecto para 32 bits)" -ForegroundColor Red
                $archCorrect = $false
            } else {
                Write-Host "  [ADVERTENCIA] NSIS: $($file.Name) - No contiene indicador de arquitectura" -ForegroundColor Yellow
            }
        }
        
        if (-not $archCorrect) {
            Write-Host "ADVERTENCIA: Algunos archivos tienen nombres incorrectos para la arquitectura de 32 bits" -ForegroundColor Red
        } else {
            Write-Host "Verificacion de nombres: Todos los archivos tienen nombres correctos para 32 bits" -ForegroundColor Green
        }
        
        # Mostrar tamano del bundle
        $bundleSize = (Get-ChildItem -Path $bundlePath -Recurse | Measure-Object -Property Length -Sum).Sum
        $bundleSizeMB = [math]::Round($bundleSize / 1MB, 2)
        Write-Host "Tamano total del bundle: $bundleSizeMB MB" -ForegroundColor Cyan
    } else {
        Write-Host "No se encontro el directorio del bundle" -ForegroundColor Red
    }

} catch {
    Write-Host "Error durante el build: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}

Write-Host "Script completado" -ForegroundColor Green

