# Release Packaging Script for Nexus Mods and GitHub
# Runs optimized compilation and generates portable .zip for distribution

$ErrorActionPreference = "Stop"

Write-Host "==========================================================" -ForegroundColor Cyan
Write-Host "  Creating Release Package - SoulsConflict   " -ForegroundColor Yellow
Write-Host "==========================================================" -ForegroundColor Cyan

$rootDir = $PSScriptRoot
if (-not $rootDir) { $rootDir = Get-Location }

Set-Location $rootDir

Write-Host "[1/4] Compiling in optimized Release mode..." -ForegroundColor Green
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Error "Cargo build failed."
    exit $LASTEXITCODE
}

$distRoot = Join-Path $rootDir "dist"
$pkgFolder = Join-Path $distRoot "SoulsConflict-v2.0.2-Portable"
$zipPath = Join-Path $distRoot "SoulsConflict-v2.0.2-Portable.zip"

Write-Host "[2/4] Preparing clean portable structure in $pkgFolder..." -ForegroundColor Green

# Kill residual processes if any
Stop-Process -Name 'SoulsConflict', 'souls_conflict' -Force -ErrorAction SilentlyContinue
Stop-Process -Name 'msedgewebview2' -Force -ErrorAction SilentlyContinue
Start-Sleep -Milliseconds 300

if (Test-Path $pkgFolder) {
    # Remove normal items
    Get-ChildItem -Path $pkgFolder -Exclude "*.WebView2" | Remove-Item -Recurse -Force -ErrorAction SilentlyContinue
    # Try to remove .WebView2 folder if possible
    Remove-Item -Path "$pkgFolder\*.WebView2" -Recurse -Force -ErrorAction SilentlyContinue
}

New-Item -ItemType Directory -Path $pkgFolder -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $pkgFolder "mods") -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $pkgFolder "merged") -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $pkgFolder "vanilla_backup") -Force | Out-Null

# Copy executable
$exeSrc = Join-Path $rootDir "target\release\souls_conflict.exe"
$exeDest = Join-Path $pkgFolder "SoulsConflict.exe"
Copy-Item $exeSrc $exeDest -Force

# Copy manuals and instructions
Copy-Item (Join-Path $rootDir "README_NEXUS.txt") (Join-Path $pkgFolder "README.txt") -Force
Copy-Item (Join-Path $rootDir "mods\HOW_TO_ADD_MODS.txt") (Join-Path $pkgFolder "mods\HOW_TO_ADD_MODS.txt") -Force

Write-Host "[3/4] Compressing final .zip package..." -ForegroundColor Green
if (Test-Path $zipPath) { Remove-Item -Force $zipPath }

# Create temporary staging folder to ensure a 100% clean ZIP
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

Write-Host "[4/4] Package ready successfully!" -ForegroundColor Cyan
$zipSizeMB = [math]::Round((Get-Item $zipPath).Length / 1MB, 2)

Write-Host "----------------------------------------------------------" -ForegroundColor Yellow
Write-Host "ZIP File Generated: $zipPath" -ForegroundColor White
Write-Host "Package Size: $zipSizeMB MB" -ForegroundColor White
Write-Host "Ready for upload on the Nexus Mods mod page and GitHub Releases!" -ForegroundColor Green
Write-Host "----------------------------------------------------------" -ForegroundColor Yellow
