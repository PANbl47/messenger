$ErrorActionPreference = "Stop"

function Invoke-NodeTestIfPresent {
    param(
        [Parameter(Mandatory = $true)]
        [string]$RelativePath
    )

    if (-not (Test-Path $RelativePath)) {
        return
    }

    Push-Location $RelativePath
    try {
        if (-not (Test-Path "package.json")) {
            Write-Host "Skipping $RelativePath because package.json is missing."
            return
        }

        if (Test-Path "pnpm-lock.yaml") {
            pnpm test
            return
        }

        if (Test-Path "yarn.lock") {
            yarn test
            return
        }

        npm test
    }
    finally {
        Pop-Location
    }
}

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$repoRoot = Resolve-Path (Join-Path $scriptDir "..")

Push-Location $repoRoot
try {
    cargo test --workspace

    @(
        "apps/web",
        "apps/desktop",
        "apps/mobile",
        "client/web",
        "client/mobile"
    ) | ForEach-Object {
        Invoke-NodeTestIfPresent -RelativePath $_
    }
}
finally {
    Pop-Location
}
