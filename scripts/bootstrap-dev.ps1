$ErrorActionPreference = "Stop"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$repoRoot = Resolve-Path (Join-Path $scriptDir "..")

Push-Location $repoRoot
try {
    rustup toolchain install stable --profile minimal
    rustup component add clippy rustfmt
    cargo fetch
}
finally {
    Pop-Location
}
