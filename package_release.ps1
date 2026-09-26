# Script de Empacotamento de Release para Nexus Mods
# Executa compilação otimizada e gera o .zip portátil pronto para distribuição

$ErrorActionPreference = "Stop"

Write-Host "==========================================================" -ForegroundColor Cyan
Write-Host "  Criando Pacote de Release Nexus Mods - SoulsConflict   " -ForegroundColor Yellow
Write-Host "==========================================================" -ForegroundColor Cyan

$rootDir = $PSScriptRoot
if (-not $rootDir) { $rootDir = Get-Location }

Set-Location $rootDir

Write-Host "[1/4] Compilando em modo Release otimizado..." -ForegroundColor Green
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Error "Falha na compilacao do Cargo."
    exit $LASTEXITCODE
}

$distRoot = Join-Path $rootDir "dist"
$pkgFolder = Join-Path $distRoot "SoulsConflict-v2.0-Portavel"
$zipPath = Join-Path $distRoot "SoulsConflict-v2.0-Portavel.zip"

Write-Host "[2/4] Preparando estrutura portatil limpa em $pkgFolder..." -ForegroundColor Green

# Finalizar processos residuais se houver
Stop-Process -Name 'SoulsConflict', 'souls_conflict' -Force -ErrorAction SilentlyContinue
Stop-Process -Name 'msedgewebview2' -Force -ErrorAction SilentlyContinue
Start-Sleep -Milliseconds 300

if (Test-Path $pkgFolder) {
    # Remove itens normais
    Get-ChildItem -Path $pkgFolder -Exclude "*.WebView2" | Remove-Item -Recurse -Force -ErrorAction SilentlyContinue
    # Tenta remover pasta .WebView2 se possivel
    Remove-Item -Path "$pkgFolder\*.WebView2" -Recurse -Force -ErrorAction SilentlyContinue
}

New-Item -ItemType Directory -Path $pkgFolder -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $pkgFolder "mods") -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $pkgFolder "merged") -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $pkgFolder "vanilla_backup") -Force | Out-Null

# Copiar executavel
$exeSrc = Join-Path $rootDir "target\release\souls_conflict.exe"
$exeDest = Join-Path $pkgFolder "SoulsConflict.exe"
Copy-Item $exeSrc $exeDest -Force

# Copiar manuais e instrucoes
Copy-Item (Join-Path $rootDir "README_NEXUS.txt") (Join-Path $pkgFolder "README.txt") -Force
Copy-Item (Join-Path $rootDir "mods\HOW_TO_ADD_MODS.txt") (Join-Path $pkgFolder "mods\HOW_TO_ADD_MODS.txt") -Force

Write-Host "[3/4] Compactando pacote final .zip..." -ForegroundColor Green
if (Test-Path $zipPath) { Remove-Item -Force $zipPath }

# Criar pasta temporaria de empacotamento para garantir ZIP 100% puro
$stageZipDir = Join-Path $distRoot "stage_zip"
if (Test-Path $stageZipDir) { Remove-Item -Recurse -Force $stageZipDir }
New-Item -ItemType Directory -Path $stageZipDir -Force | Out-Null

Copy-Item $exeDest (Join-Path $stageZipDir "SoulsConflict.exe") -Force
Copy-Item (Join-Path $pkgFolder "README.txt") (Join-Path $stageZipDir "README.txt") -Force
Copy-Item -Recurse (Join-Path $pkgFolder "mods") (Join-Path $stageZipDir "mods") -Force
New-Item -ItemType Directory -Path (Join-Path $stageZipDir "merged") -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $stageZipDir "vanilla_backup") -Force | Out-Null

Compress-Archive -Path "$stageZipDir\*" -DestinationPath $zipPath -CompressionLevel Optimal
Remove-Item -Recurse -Force $stageZipDir -ErrorAction SilentlyContinue

Write-Host "[4/4] Pacote pronto com sucesso!" -ForegroundColor Cyan
$zipSizeMB = [math]::Round((Get-Item $zipPath).Length / 1MB, 2)

Write-Host "----------------------------------------------------------" -ForegroundColor Yellow
Write-Host "Arquivo ZIP Gerado: $zipPath" -ForegroundColor White
Write-Host "Tamanho do Pacote: $zipSizeMB MB" -ForegroundColor White
Write-Host "Pronto para upload na pagina do mod na Nexus Mods!" -ForegroundColor Green
Write-Host "----------------------------------------------------------" -ForegroundColor Yellow
