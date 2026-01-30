# Script para generar el paquete de distribución (para el desarrollador)

$ErrorActionPreference = "Stop"
$ProjectRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
$DistDir = Join-Path $ProjectRoot "dist"
$TargetRelease = Join-Path $ProjectRoot "target\release\oxiproc.exe"

Write-Host "Compilando versión release..." -ForegroundColor Cyan
Set-Location $ProjectRoot
cargo build --release

if (-not (Test-Path $TargetRelease)) {
    Write-Error "Error: No se encontró el ejecutable compilado en $TargetRelease"
    exit 1
}

# Preparar carpeta de distribución
if (Test-Path $DistDir) { Remove-Item $DistDir -Recurse -Force }
New-Item -ItemType Directory -Path $DistDir | Out-Null

# Copiar archivos
Copy-Item $TargetRelease -Destination $DistDir
Copy-Item (Join-Path $PSScriptRoot "install.ps1") -Destination $DistDir

Write-Host "Paquete generado en: $DistDir" -ForegroundColor Green
Write-Host "Contenido:"
Get-ChildItem $DistDir | Select-Object Name

Write-Host "`nPara distribuir:" -ForegroundColor Yellow
Write-Host "1. Comprime el contenido de la carpeta 'dist' en un ZIP."
Write-Host "2. Envía el ZIP."
Write-Host "3. El usuario solo debe descomprimir y ejecutar 'install.ps1' (clic derecho -> Ejecutar con PowerShell)."
