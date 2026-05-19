# Strata-Reader — local dev orchestrator (Windows / PowerShell).
#
# Replaces `docker compose up` for environments without Docker. Idempotent:
# safe to run multiple times. See docs/task/tareas.md T0.5.A0.5.3.
#
# Usage:
#   .\scripts\dev_up.ps1                  # start Ollama + pull models
#   .\scripts\dev_up.ps1 -WithServer      # also launch `strata-server` in bg
#   .\scripts\dev_up.ps1 -SkipModels      # skip model pull (fast restart)

[CmdletBinding()]
param(
    [switch] $WithServer,
    [switch] $SkipModels,
    [string] $OllamaEndpoint = $(if ($env:STRATA_OLLAMA_URL) { $env:STRATA_OLLAMA_URL } else { 'http://localhost:11434' })
)

$ErrorActionPreference = 'Stop'
$ScriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptRoot '..')
$StateDir = Join-Path $env:LOCALAPPDATA 'strata'
New-Item -ItemType Directory -Force -Path $StateDir | Out-Null
$PidFile = Join-Path $StateDir 'dev.pid'

function Write-Section { param([string] $Msg) Write-Host "`n==> $Msg" -ForegroundColor Cyan }
function Write-Ok       { param([string] $Msg) Write-Host "    ok  $Msg" -ForegroundColor Green }
function Write-Warn     { param([string] $Msg) Write-Host "    !!  $Msg" -ForegroundColor Yellow }

function Test-OllamaInstalled {
    if (-not (Get-Command ollama -ErrorAction SilentlyContinue)) {
        Write-Host @"
ollama CLI not found on PATH.
Install (user-scope, no admin required):
  - Download:  https://ollama.com/download/windows
  - or:        winget install Ollama.Ollama --scope user
After installing, re-run this script.
"@ -ForegroundColor Red
        exit 127
    }
    Write-Ok "ollama $(ollama --version 2>&1 | Select-Object -First 1)"
}

function Test-OllamaReachable {
    param([int] $TimeoutSec = 3)
    try {
        Invoke-RestMethod -Uri "$OllamaEndpoint/api/tags" -TimeoutSec $TimeoutSec | Out-Null
        return $true
    } catch {
        return $false
    }
}

function Start-OllamaIfNeeded {
    if (Test-OllamaReachable) {
        Write-Ok "Ollama already reachable at $OllamaEndpoint"
        return
    }
    Write-Section "Starting `ollama serve` in background"
    $proc = Start-Process -FilePath 'ollama' -ArgumentList 'serve' -PassThru -WindowStyle Hidden
    Write-Ok "PID $($proc.Id)"
    for ($i = 1; $i -le 15; $i++) {
        Start-Sleep -Seconds 1
        if (Test-OllamaReachable) {
            Write-Ok "responding after $i s"
            return
        }
    }
    Write-Warn "Ollama did not respond within 15s — check `ollama serve` output manually"
    exit 2
}

function Invoke-ModelPull {
    if ($SkipModels) {
        Write-Warn "Skipping model pull (-SkipModels)"
        return
    }
    Write-Section "Pulling required models"
    & (Join-Path $ScriptRoot 'pull_models.ps1')
    if ($LASTEXITCODE -ne 0) {
        Write-Warn "pull_models.ps1 exited with code $LASTEXITCODE"
        exit $LASTEXITCODE
    }
}

function Start-StrataServer {
    if (-not $WithServer) { return }
    Write-Section "Building and launching strata-server"
    Push-Location $RepoRoot
    try {
        & cargo build -p strata-server --release
        if ($LASTEXITCODE -ne 0) {
            Write-Warn "cargo build failed"
            exit 3
        }
        $binary = Join-Path $RepoRoot 'target\release\strata-server.exe'
        if (-not (Test-Path $binary)) {
            Write-Warn "Built binary not found at $binary"
            exit 4
        }
        $proc = Start-Process -FilePath $binary -PassThru -WindowStyle Hidden
        $proc.Id | Out-File -FilePath $PidFile -Encoding ascii
        Write-Ok "strata-server PID $($proc.Id) (pid file: $PidFile)"
        Write-Ok "Reachable at http://localhost:8080"
    } finally {
        Pop-Location
    }
}

function Show-Summary {
    Write-Section "Summary"
    try {
        $models = (Invoke-RestMethod "$OllamaEndpoint/api/tags").models | ForEach-Object { $_.name }
        Write-Host "  ollama endpoint : $OllamaEndpoint"
        Write-Host "  ollama models   : $($models -join ', ')"
    } catch {
        Write-Warn "Could not read /api/tags"
    }
    if (Test-Path $PidFile) {
        Write-Host "  strata-server   : pid $(Get-Content $PidFile)"
    } else {
        Write-Host "  strata-server   : not running (use -WithServer to launch)"
    }
    Write-Host ""
    Write-Host "Tip: run `cargo run -p strata-cli -- doctor` to validate the full env." -ForegroundColor DarkGray
}

Write-Section "Strata-Reader dev environment"
Test-OllamaInstalled
Start-OllamaIfNeeded
Invoke-ModelPull
Start-StrataServer
Show-Summary
