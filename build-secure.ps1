#!/usr/bin/env pwsh

# Script para construir la aplicación Tauri en modo producción
# Este script verifica que la configuración segura funcione correctamente

Write-Host "🔧 Construyendo aplicación Tauri..." -ForegroundColor Blue

# Cambiar al directorio del proyecto
Set-Location "e:\repos\Toscanini-IngSoft"

try {
    # Construir la aplicación
    Write-Host "📦 Ejecutando build de producción..." -ForegroundColor Yellow
    npm run tauri build

    # Verificar que el .env no esté en el bundle
    $bundlePath = "src-tauri\target\release\bundle"
    if (Test-Path $bundlePath) {
        Write-Host "✅ Bundle creado en: $bundlePath" -ForegroundColor Green
        
        # Buscar archivos .env en el bundle
        $envFiles = Get-ChildItem -Path $bundlePath -Recurse -Name "*.env" -ErrorAction SilentlyContinue
        
        if ($envFiles.Count -eq 0) {
            Write-Host "🔒 SEGURIDAD: No se encontraron archivos .env en el bundle - ¡Excelente!" -ForegroundColor Green
        } else {
            Write-Host "⚠️  ADVERTENCIA: Se encontraron archivos .env en el bundle:" -ForegroundColor Red
            $envFiles | ForEach-Object { Write-Host "   - $_" -ForegroundColor Red }
        }
        
        # Mostrar tamaño del bundle
        $bundleSize = (Get-ChildItem -Path $bundlePath -Recurse | Measure-Object -Property Length -Sum).Sum
        $bundleSizeMB = [math]::Round($bundleSize / 1MB, 2)
        Write-Host "📊 Tamaño total del bundle: $bundleSizeMB MB" -ForegroundColor Cyan
    } else {
        Write-Host "❌ No se encontró el directorio del bundle" -ForegroundColor Red
    }

} catch {
    Write-Host "❌ Error durante el build: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}

Write-Host "✅ Script completado" -ForegroundColor Green
