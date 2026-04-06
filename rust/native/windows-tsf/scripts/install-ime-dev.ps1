param(
  [string]$WorkspaceRoot = (Resolve-Path (Join-Path $PSScriptRoot '..\..\..')).Path
)

$ErrorActionPreference = 'Stop'

Set-Location $WorkspaceRoot

Write-Host '1/2 Build TSF release DLL'
cargo build --manifest-path .\rust\Cargo.toml -p windows-tsf --release

$dllPath = Join-Path $WorkspaceRoot 'rust\target\release\windows_tsf.dll'
if (-not (Test-Path -LiteralPath $dllPath)) {
  throw "Missing release DLL: $dllPath"
}

Write-Host '2/2 Register development DLL'
powershell -ExecutionPolicy Bypass -File .\rust\native\windows-tsf\scripts\register-ime.ps1 -DllPath $dllPath

Write-Host ''
Write-Host 'Development install script completed.'
