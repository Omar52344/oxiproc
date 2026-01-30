# Installer simples for Oxiproc
# Este script copia el ejecutable a una carpeta en LocalAppData y la añade al PATH.

$ErrorActionPreference = "Stop"

$AppName = "oxiproc"
$ExeName = "$AppName.exe"
$InstallDir = Join-Path $env:LOCALAPPDATA $AppName
$SourceExe = Join-Path $PSScriptRoot $ExeName

# 1. Verificar que el ejecutable existe en la misma carpeta que el script
if (-not (Test-Path $SourceExe)) {
    Write-Error "No se encontró $ExeName. Asegúrate de que $ExeName esté en la misma carpeta que este script."
    exit 1
}

# 2. Crear directorio de instalación
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir | Out-Null
    Write-Host "Directorio creado: $InstallDir" -ForegroundColor Green
}

# 3. Copiar ejecutable
Copy-Item -Path $SourceExe -Destination $InstallDir -Force
Write-Host "$ExeName instalado en $InstallDir" -ForegroundColor Green

# 4. Añadir al PATH (si no está ya)
$CurrentPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($CurrentPath -split ';' -notcontains $InstallDir) {
    [Environment]::SetEnvironmentVariable("Path", "$CurrentPath;$InstallDir", "User")
    Write-Host "Ruta añadida al PATH del usuario." -ForegroundColor Green
    Write-Host "Por favor, reinicia tu terminal para usar '$AppName' desde cualquier lugar." -ForegroundColor Yellow
} else {
    Write-Host "La ruta ya está en el PATH." -ForegroundColor Cyan
}

Write-Host "¡Instalación completada con éxito!" -ForegroundColor Green
Read-Host "Presiona Enter para salir"
