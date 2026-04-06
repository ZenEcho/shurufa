param(
  [string]$DllPath = (Join-Path $PSScriptRoot '..\..\..\target\release\windows_tsf.dll'),
  [string]$DisplayName = '书入法输入法',
  [string]$Clsid = '{9f1e7af0-7981-4c9c-9a6d-26d1b2d11810}',
  [string]$ProfileGuid = '{4bd8d561-b3f4-4932-a9f5-7c3d7c1cf4de}',
  [string]$IconPath = '%ProgramFiles%\\Shurufa\\shurufa-ime.dll',
  [int]$LangId = 0x0804
)

$ErrorActionPreference = 'Stop'

$resolvedDll = Resolve-Path -LiteralPath $DllPath -ErrorAction Stop
$clsidKey = "HKCU:\Software\Classes\CLSID\$Clsid"
$inprocKey = "$clsidKey\InprocServer32"
$tipKey = "HKCU:\Software\Microsoft\CTF\TIP\$Clsid"
$profileKey = "$tipKey\LanguageProfile\0x{0:x4}\$ProfileGuid" -f $LangId

Write-Host "Registering TSF component: $resolvedDll"

New-Item -Path $clsidKey -Force | Out-Null
Set-Item -Path $clsidKey -Value $DisplayName

New-Item -Path $inprocKey -Force | Out-Null
Set-Item -Path $inprocKey -Value $resolvedDll
Set-ItemProperty -Path $inprocKey -Name 'ThreadingModel' -Value 'Apartment'

New-Item -Path $tipKey -Force | Out-Null
New-Item -Path $profileKey -Force | Out-Null
Set-ItemProperty -Path $profileKey -Name 'Description' -Value $DisplayName
Set-ItemProperty -Path $profileKey -Name 'IconFile' -Value $IconPath
Set-ItemProperty -Path $profileKey -Name 'IconIndex' -Value 0

$regsvr32 = Join-Path $env:SystemRoot 'System32\regsvr32.exe'
& $regsvr32 /s $resolvedDll

if ($LASTEXITCODE -ne 0) {
  throw 'regsvr32 failed while calling DllRegisterServer.'
}

Write-Host ''
Write-Host 'Registry keys and DllRegisterServer completed.'
Write-Host "CLSID: $Clsid"
Write-Host "Profile GUID: $ProfileGuid"
Write-Host ("Language ID: 0x{0:x4}" -f $LangId)
Write-Host 'Note: text write-back is wired for MVP commit flow; candidate UI is still pending.'
