#!/usr/bin/env pwsh

# Script para construir la aplicacion Tauri en modo produccion para 64 bits
# Este script verifica que la configuracion segura funcione correctamente

Write-Host "Construyendo aplicacion Tauri para 64 bits..." -ForegroundColor Blue

# Cambiar al directorio del proyecto
Set-Location "e:\repos\Toscanini-IngSoft"

try {
    # Guardar el archivo de configuracion de cargo si existe
    $cargoConfigPath = "src-tauri\.cargo\config.toml"
    $cargoConfigBackup = "src-tauri\.cargo\config.toml.backup"
    $configExists = Test-Path $cargoConfigPath
    
    if ($configExists) {
        Write-Host "Archivo de configuracion de Cargo encontrado. Haciendo backup temporal..." -ForegroundColor Yellow
        Copy-Item -Path $cargoConfigPath -Destination $cargoConfigBackup -Force
        Remove-Item -Path $cargoConfigPath -Force
        Write-Host "Configuracion temporalmente deshabilitada para compilacion de 64 bits" -ForegroundColor Cyan
    }
    
    # Limpiar cualquier variable de entorno de target anterior
    if ($env:CARGO_BUILD_TARGET) {
        Remove-Item Env:\CARGO_BUILD_TARGET
    }
    
    Write-Host "Target configurado: x86_64-pc-windows-msvc (64 bits - por defecto)" -ForegroundColor Cyan
    
    # Construir la aplicacion
    Write-Host "Ejecutando build de produccion..." -ForegroundColor Yellow
    npm run tauri build

    # Restaurar el archivo de configuracion si existia
    if ($configExists -and (Test-Path $cargoConfigBackup)) {
        Write-Host "Restaurando configuracion de Cargo..." -ForegroundColor Yellow
        Copy-Item -Path $cargoConfigBackup -Destination $cargoConfigPath -Force
        Remove-Item -Path $cargoConfigBackup -Force
        Write-Host "Configuracion restaurada" -ForegroundColor Green
    }

    # Verificar que el .env no este en el bundle
    # Para 64 bits, la ruta es la estandar
    $bundlePath = "src-tauri\target\release\bundle"
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
        
        # Verificar nombres de archivos generados (deben contener x64 para 64 bits)
        Write-Host "Verificando nombres de archivos generados..." -ForegroundColor Cyan
        $msiFiles = Get-ChildItem -Path "$bundlePath\msi" -Filter "*.msi" -ErrorAction SilentlyContinue
        $nsisFiles = Get-ChildItem -Path "$bundlePath\nsis" -Filter "*.exe" -ErrorAction SilentlyContinue
        
        $archCorrect = $true
        
        foreach ($file in $msiFiles) {
            if ($file.Name -match "x64") {
                Write-Host "  [OK] MSI: $($file.Name) - Contiene x64 (correcto para 64 bits)" -ForegroundColor Green
            } elseif ($file.Name -match "x32") {
                Write-Host "  [ERROR] MSI: $($file.Name) - Contiene x32 (incorrecto para 64 bits)" -ForegroundColor Red
                $archCorrect = $false
            } else {
                Write-Host "  [ADVERTENCIA] MSI: $($file.Name) - No contiene indicador de arquitectura" -ForegroundColor Yellow
            }
        }
        
        foreach ($file in $nsisFiles) {
            if ($file.Name -match "x64") {
                Write-Host "  [OK] NSIS: $($file.Name) - Contiene x64 (correcto para 64 bits)" -ForegroundColor Green
            } elseif ($file.Name -match "x32") {
                Write-Host "  [ERROR] NSIS: $($file.Name) - Contiene x32 (incorrecto para 64 bits)" -ForegroundColor Red
                $archCorrect = $false
            } else {
                Write-Host "  [ADVERTENCIA] NSIS: $($file.Name) - No contiene indicador de arquitectura" -ForegroundColor Yellow
            }
        }
        
        if (-not $archCorrect) {
            Write-Host "ADVERTENCIA: Algunos archivos tienen nombres incorrectos para la arquitectura de 64 bits" -ForegroundColor Red
        } else {
            Write-Host "Verificacion de nombres: Todos los archivos tienen nombres correctos para 64 bits" -ForegroundColor Green
        }
        
        # Mostrar tamano del bundle
        $bundleSize = (Get-ChildItem -Path $bundlePath -Recurse | Measure-Object -Property Length -Sum).Sum
        $bundleSizeMB = [math]::Round($bundleSize / 1MB, 2)
        Write-Host "Tamano total del bundle: $bundleSizeMB MB" -ForegroundColor Cyan
    } else {
        Write-Host "No se encontro el directorio del bundle" -ForegroundColor Red
    }

} catch {
    # Asegurarse de restaurar el archivo de configuracion incluso si hay error
    $cargoConfigPath = "src-tauri\.cargo\config.toml"
    $cargoConfigBackup = "src-tauri\.cargo\config.toml.backup"
    if ((Test-Path $cargoConfigBackup) -and -not (Test-Path $cargoConfigPath)) {
        Write-Host "Restaurando configuracion de Cargo tras error..." -ForegroundColor Yellow
        Copy-Item -Path $cargoConfigBackup -Destination $cargoConfigPath -Force
        Remove-Item -Path $cargoConfigBackup -Force
    }
    
    Write-Host "Error durante el build: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}

Write-Host "Script completado" -ForegroundColor Green

