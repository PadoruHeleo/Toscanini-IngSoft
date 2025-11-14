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

