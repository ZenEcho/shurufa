$ErrorActionPreference = 'Stop'
[void][System.Console]::OutputEncoding
chcp 65001 > $null
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
$OutputEncoding = [System.Text.Encoding]::UTF8

function Invoke-Step {
  param(
    [Parameter(Mandatory = $true)]
    [string]$Label,
    [Parameter(Mandatory = $true)]
    [scriptblock]$Action
  )

  Write-Host $Label
  & $Action
  if ($LASTEXITCODE -ne 0) {
    throw "Step failed: $Label"
  }
}

Set-Location (Join-Path $PSScriptRoot '..')

Invoke-Step -Label '1/4 Run Rust tests' -Action {
  cargo test --manifest-path rust\Cargo.toml -p ime-dict -p ime-core -p ime-service -p windows-tsf
}

if ((Test-Path node_modules) -and (Test-Path apps\desktop-settings\node_modules)) {
  Invoke-Step -Label '2/4 Run frontend typecheck' -Action {
    pnpm typecheck
  }

  Invoke-Step -Label '3/4 Run desktop build' -Action {
    pnpm build
  }
} else {
  Write-Host '2/4 Skip frontend typecheck (node_modules missing in this worktree)'
  Write-Host '3/4 Skip desktop build (node_modules missing in this worktree)'
}

Invoke-Step -Label '4/4 Run typing MVP script' -Action {
  powershell -ExecutionPolicy Bypass -File .\scripts\test-typing-mvp.ps1
}

Write-Host ''
Write-Host 'MVP verification completed.'
