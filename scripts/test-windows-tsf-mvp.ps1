param(
  [switch]$Install
)

$ErrorActionPreference = 'Stop'
[void][System.Console]::OutputEncoding
chcp 65001 > $null
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
$OutputEncoding = [System.Text.Encoding]::UTF8

Set-Location (Join-Path $PSScriptRoot '..')

$dllPath = Join-Path (Get-Location) 'rust\target\release\windows_tsf.dll'
$clsid = '{9f1e7af0-7981-4c9c-9a6d-26d1b2d11810}'
$profileGuid = '{4bd8d561-b3f4-4932-a9f5-7c3d7c1cf4de}'
$clsidKey = "HKCU:\Software\Classes\CLSID\$clsid"
$tipKey = "HKCU:\Software\Microsoft\CTF\TIP\$clsid"

Write-Host '1/4 Build Windows TSF release DLL'
cargo build --manifest-path .\rust\Cargo.toml -p windows-tsf --release

if (-not (Test-Path -LiteralPath $dllPath)) {
  throw "Missing DLL: $dllPath"
}

if ($Install) {
  Write-Host '2/4 Install development IME'
  powershell -ExecutionPolicy Bypass -File .\rust\native\windows-tsf\scripts\install-ime-dev.ps1
} else {
  Write-Host '2/4 Skip install'
}

Write-Host '3/4 Check local registration state'
Write-Host ("DLL path: {0}" -f $dllPath)
Write-Host ("CLSID key exists: {0}" -f (Test-Path $clsidKey))
Write-Host ("TIP key exists: {0}" -f (Test-Path $tipKey))
Write-Host ("Profile GUID: {0}" -f $profileGuid)

Write-Host '4/4 Manual typing checklist'
Write-Host '  1. Open Settings > Time & language > Language & region.'
Write-Host '  2. Confirm that 书入法输入法 appears in the input method list.'
Write-Host '  3. Switch to 书入法输入法.'
Write-Host '  4. Open Notepad or VS Code.'
Write-Host '  5. Type s, then press 1. Expected commit: shurufa.'
Write-Host '  6. Press F2, then type x. Expected commit: x.'
Write-Host '  7. Check Backspace, Space, Enter, and Escape behavior.'

Write-Host ''
Write-Host 'Windows TSF MVP test script completed.'
