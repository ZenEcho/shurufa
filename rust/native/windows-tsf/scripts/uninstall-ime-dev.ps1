param(
  [string]$WorkspaceRoot = (Resolve-Path (Join-Path $PSScriptRoot '..\..\..')).Path
)

$ErrorActionPreference = 'Stop'

Set-Location $WorkspaceRoot

$dllPath = Join-Path $WorkspaceRoot 'rust\target\release\windows_tsf.dll'
if (-not (Test-Path -LiteralPath $dllPath)) {
  throw "Missing release DLL: $dllPath"
}

Write-Host 'Unregister development DLL'
powershell -ExecutionPolicy Bypass -File .\rust\native\windows-tsf\scripts\unregister-ime.ps1 -DllPath $dllPath

Write-Host ''
Write-Host 'Development uninstall script completed.'
