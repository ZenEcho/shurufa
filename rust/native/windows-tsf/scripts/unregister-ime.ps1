param(
  [string]$DllPath = (Join-Path $PSScriptRoot '..\..\..\target\release\windows_tsf.dll'),
  [string]$Clsid = '{9f1e7af0-7981-4c9c-9a6d-26d1b2d11810}',
  [string]$ProfileGuid = '{4bd8d561-b3f4-4932-a9f5-7c3d7c1cf4de}',
  [int]$LangId = 0x0804
)

$ErrorActionPreference = 'Stop'

$resolvedDll = Resolve-Path -LiteralPath $DllPath -ErrorAction Stop
$clsidKey = "HKCU:\Software\Classes\CLSID\$Clsid"
$tipKey = "HKCU:\Software\Microsoft\CTF\TIP\$Clsid"
$profileKey = "$tipKey\LanguageProfile\0x{0:x4}\$ProfileGuid" -f $LangId

Write-Host "Unregistering TSF component: $resolvedDll"

$regsvr32 = Join-Path $env:SystemRoot 'System32\regsvr32.exe'
& $regsvr32 /u /s $resolvedDll

if ($LASTEXITCODE -ne 0) {
  throw 'regsvr32 failed while calling DllUnregisterServer.'
}

if (Test-Path -LiteralPath $profileKey) {
  Remove-Item -LiteralPath $profileKey -Recurse -Force
}

if (Test-Path -LiteralPath $tipKey) {
  Remove-Item -LiteralPath $tipKey -Recurse -Force
}

if (Test-Path -LiteralPath $clsidKey) {
  Remove-Item -LiteralPath $clsidKey -Recurse -Force
}

Write-Host ''
Write-Host 'DllUnregisterServer and registry cleanup completed.'
