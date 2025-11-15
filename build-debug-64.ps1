#!/usr/bin/env pwsh

# Script para construir la aplicacion Tauri en modo DEBUG para 64 bits
# Este script permite depurar problemas sin optimizaciones

Write-Host "Construyendo aplicacion Tauri en modo DEBUG para 64 bits..." -ForegroundColor Blue

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
    
    # Construir el frontend primero
    Write-Host "Construyendo frontend..." -ForegroundColor Cyan
    npm run build
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Error al construir el frontend" -ForegroundColor Red
        throw "Frontend build failed"
    }
    
    # Construir la aplicacion en modo DEBUG
    Write-Host "Ejecutando build en modo DEBUG..." -ForegroundColor Yellow
    Write-Host "NOTA: El ejecutable estara en src-tauri\target\debug\tauri-app.exe" -ForegroundColor Cyan
    
    # Usar el flag --debug de Tauri para compilar en modo debug
    npm run tauri build -- --debug
    
    # Restaurar el archivo de configuracion si existia
    if ($configExists -and (Test-Path $cargoConfigBackup)) {
        Write-Host "Restaurando configuracion de Cargo..." -ForegroundColor Yellow
        Copy-Item -Path $cargoConfigBackup -Destination $cargoConfigPath -Force
        Remove-Item -Path $cargoConfigBackup -Force
        Write-Host "Configuracion restaurada" -ForegroundColor Green
    }
    
    # Verificar ubicacion del ejecutable
    $debugExe = "src-tauri\target\debug\tauri-app.exe"
    $debugBundleExe = "src-tauri\target\debug\bundle\msi\Toscanini_0.1.0_x64_en-US.msi"
    
    if (Test-Path $debugExe) {
        Write-Host "`n========================================" -ForegroundColor Green
        Write-Host "Build DEBUG completado exitosamente!" -ForegroundColor Green
        Write-Host "========================================" -ForegroundColor Green
        Write-Host "Ejecutable DEBUG: $debugExe" -ForegroundColor Cyan
        
        if (Test-Path $debugBundleExe) {
            Write-Host "Bundle DEBUG: $debugBundleExe" -ForegroundColor Cyan
        }
        
        Write-Host "`nPara depurar:" -ForegroundColor Yellow
        Write-Host "1. Abre Visual Studio Code o tu debugger favorito" -ForegroundColor White
        Write-Host "2. Adjunta el debugger al proceso tauri-app.exe" -ForegroundColor White
        Write-Host "3. O ejecuta directamente desde la terminal para ver los logs" -ForegroundColor White
        Write-Host "`nPara ejecutar directamente:" -ForegroundColor Yellow
        Write-Host "  .\$debugExe" -ForegroundColor White
        Write-Host "`nNOTA: El ejecutable en modo debug es mas grande y lento," -ForegroundColor Yellow
        Write-Host "      pero incluye simbolos de depuracion completos." -ForegroundColor Yellow
    } else {
        Write-Host "`nADVERTENCIA: No se encontro el ejecutable en la ubicacion esperada" -ForegroundColor Yellow
        Write-Host "Busca en: src-tauri\target\debug\" -ForegroundColor Cyan
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

Write-Host "`nScript completado" -ForegroundColor Green

