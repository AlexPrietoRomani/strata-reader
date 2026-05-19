# Pull the Ollama models strata-reader depends on (Windows / PowerShell).
$ErrorActionPreference = 'Stop'

$OllamaEndpoint = if ($env:STRATA_OLLAMA_URL) { $env:STRATA_OLLAMA_URL } else { 'http://localhost:11434' }
$Models = @(
    'qwen2.5vl:7b',
    'minicpm-v:8b',
    'llama3.2-vision:11b'
)

if (-not (Get-Command ollama -ErrorAction SilentlyContinue)) {
    Write-Error "ollama CLI not found. Install from https://ollama.com"
    exit 127
}

for ($i = 1; $i -le 5; $i++) {
    try {
        Invoke-RestMethod -Uri "$OllamaEndpoint/api/tags" -TimeoutSec 3 | Out-Null
        break
    } catch {
        Write-Host "Waiting for Ollama at $OllamaEndpoint (attempt $i/5)..."
        Start-Sleep -Seconds 3
    }
}

foreach ($model in $Models) {
    Write-Host "==> Pulling $model"
    for ($attempt = 1; $attempt -le 3; $attempt++) {
        & ollama pull $model
        if ($LASTEXITCODE -eq 0) { break }
        Write-Host "  retry $attempt/3 for $model"
        Start-Sleep -Seconds 5
    }
}

Write-Host "Done. Available models:"
& ollama list
