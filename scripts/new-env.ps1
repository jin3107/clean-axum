# Creates .env from .env.example with a random JWT_SECRET. Never overwrites.
$ErrorActionPreference = 'Stop'
$root = Split-Path $PSScriptRoot -Parent
$target = Join-Path $root '.env'
if (Test-Path $target) { throw '.env already exists, nothing changed' }

$bytes = [byte[]]::new(32)
[System.Security.Cryptography.RandomNumberGenerator]::Create().GetBytes($bytes)
$secret = -join ($bytes | ForEach-Object { $_.ToString('x2') })

$content = Get-Content (Join-Path $root '.env.example') -Raw
$content = $content -replace '(?m)^JWT_SECRET=.*?(?=\r?$)', "JWT_SECRET=$secret"
[System.IO.File]::WriteAllText($target, $content)
Write-Host 'Created .env'
