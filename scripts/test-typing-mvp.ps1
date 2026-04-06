$ErrorActionPreference = 'Stop'
[void][System.Console]::OutputEncoding
chcp 65001 > $null
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
$OutputEncoding = [System.Text.Encoding]::UTF8

$manifestPath = Join-Path $PSScriptRoot '..\rust\Cargo.toml'

function Invoke-ImeServiceJson {
  param(
    [Parameter(Mandatory = $true)]
    [string]$Command,
    [string]$Payload
  )

  $args = @('run', '--quiet', '--manifest-path', $manifestPath, '-p', 'ime-service', '--', '--json', $Command)

  if ($Payload) {
    $args += $Payload
  }

  $result = & cargo @args

  if ($LASTEXITCODE -ne 0) {
    throw "ime-service command failed: $Command"
  }

  return $result | ConvertFrom-Json
}

$demo = Invoke-ImeServiceJson -Command 'run-typing-demo'
Write-Host "Initial mode: $($demo.initial_mode)"
Write-Host "Preedit after ni: $($demo.first_preedit)"
Write-Host "First candidate: $($demo.first_candidate)"
Write-Host "Selected commit: $($demo.selected_commit)"
Write-Host "Mode after toggle: $($demo.toggled_mode)"
Write-Host "English commit: $($demo.english_commit)"

Write-Host ''
Write-Host 'Typing MVP script completed.'
