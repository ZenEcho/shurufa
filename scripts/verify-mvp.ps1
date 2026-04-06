$ErrorActionPreference = 'Stop'
[void][System.Console]::OutputEncoding
chcp 65001 > $null
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
$OutputEncoding = [System.Text.Encoding]::UTF8

Set-Location (Join-Path $PSScriptRoot '..')

Write-Host '1/4 Run Rust tests'
cargo test --manifest-path rust\Cargo.toml -p ime-core -p ime-service -p windows-tsf

Write-Host '2/4 Run frontend typecheck'
pnpm typecheck

Write-Host '3/4 Run desktop build'
pnpm build

Write-Host '4/4 Run typing MVP script'
powershell -ExecutionPolicy Bypass -File .\scripts\test-typing-mvp.ps1

Write-Host ''
Write-Host 'MVP verification completed.'
