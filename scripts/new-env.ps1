# Creates .env from .env.example with a random JWT_SECRET and, for a server
# database, the password you type. Never overwrites. For unattended use, set the
# DB_PASSWORD environment variable instead of answering the prompt.
$ErrorActionPreference = 'Stop'
$root = Split-Path $PSScriptRoot -Parent
$target = Join-Path $root '.env'
if (Test-Path $target) { throw '.env already exists, nothing changed' }

$bytes = [byte[]]::new(32)
[System.Security.Cryptography.RandomNumberGenerator]::Create().GetBytes($bytes)
$secret = -join ($bytes | ForEach-Object { $_.ToString('x2') })

$content = Get-Content (Join-Path $root '.env.example') -Raw
$content = $content -replace '(?m)^JWT_SECRET=.*?(?=\r?$)', "JWT_SECRET=$secret"
if ($content.Contains('__DB_PASSWORD__')) {
    $password = $env:DB_PASSWORD
    if ($null -eq $password) {
        $typed = Read-Host 'Database password (input hidden)' -AsSecureString
        $password = [System.Net.NetworkCredential]::new('', $typed).Password
    }
    # percent-encode so that characters such as @ : / # keep the URL valid
    $content = $content.Replace(':__DB_PASSWORD__@', ':' + [uri]::EscapeDataString($password) + '@')
}
[System.IO.File]::WriteAllText($target, $content)
Write-Host 'Created .env'
