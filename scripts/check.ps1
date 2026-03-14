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

        npm test

        if (Test-Path "playwright.config.ts") {
            npm run build
            npm run e2e
        }
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
            & ".\gradlew.bat" testDebugUnitTest
            return
        }

        if (Test-Path "gradlew") {
            & "./gradlew" testDebugUnitTest
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

if (-not $env:JAVA_HOME) {
    $temurin = Get-ChildItem "C:\Program Files\Eclipse Adoptium" -Directory -ErrorAction SilentlyContinue |
        Sort-Object Name -Descending |
        Select-Object -First 1
    if ($temurin) {
        $env:JAVA_HOME = $temurin.FullName
    }
}

Push-Location $repoRoot
try {
    cargo test --workspace --exclude desktop-shell
    cargo check --workspace --exclude desktop-shell

    Invoke-NodeTestIfPresent -RelativePath "apps/web"
    Invoke-GradleTestIfPresent -RelativePath "apps/android"
}
finally {
    Pop-Location
}
