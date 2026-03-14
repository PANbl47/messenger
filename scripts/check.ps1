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

function Invoke-GradleTestIfPresent {
    param(
        [Parameter(Mandatory = $true)]
        [string]$RelativePath
    )

    if (-not (Test-Path $RelativePath)) {
        return
    }

    Push-Location $RelativePath
    try {
        $hasGradleProject = (Test-Path "settings.gradle.kts") -or (Test-Path "build.gradle.kts")
        if (-not $hasGradleProject) {
            Write-Host "Skipping $RelativePath because no Gradle build files were found."
            return
        }

        if ($IsWindows -and (Test-Path "gradlew.bat")) {
            & ".\gradlew.bat" test
            return
        }

        if (Test-Path "gradlew") {
            & "./gradlew" test
            return
        }

        Write-Host "Skipping $RelativePath because the Gradle wrapper is missing."
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

    Invoke-NodeTestIfPresent -RelativePath "apps/web"
    Invoke-GradleTestIfPresent -RelativePath "apps/android"
}
finally {
    Pop-Location
}
