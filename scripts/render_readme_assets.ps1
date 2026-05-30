$ErrorActionPreference = "Stop"

$root = Resolve-Path (Join-Path $PSScriptRoot "..")
$screenshots = Join-Path $root "screenshots"
$port = 5532
$process = $null
$stdout = Join-Path $env:TEMP ("sensor-health-drift-monitor-" + [guid]::NewGuid().ToString() + "-stdout.log")
$stderr = Join-Path $env:TEMP ("sensor-health-drift-monitor-" + [guid]::NewGuid().ToString() + "-stderr.log")
$edgeCandidates = @(
    "C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
    "C:\Program Files\Microsoft\Edge\Application\msedge.exe"
)

New-Item -ItemType Directory -Force -Path $screenshots | Out-Null

function Get-EdgePath {
    foreach ($candidate in $edgeCandidates) {
        if (Test-Path $candidate) {
            return $candidate
        }
    }
    throw "Microsoft Edge was not found."
}

function Wait-ForUrl {
    param([string]$Url)
    for ($i = 0; $i -lt 40; $i++) {
        try {
            Invoke-WebRequest -Uri $Url -UseBasicParsing | Out-Null
            return
        } catch {
            Start-Sleep -Milliseconds 750
        }
    }
    throw "Timed out waiting for $Url"
}

function Assert-Port-Free {
    param([int]$Port)
    $existing = Get-NetTCPConnection -LocalPort $Port -ErrorAction SilentlyContinue | Select-Object -First 1
    if ($existing) {
        $proc = Get-Process -Id $existing.OwningProcess -ErrorAction SilentlyContinue
        if ($proc) {
            throw "Port $Port is already in use by process $($proc.ProcessName) (PID $($proc.Id))."
        }
        throw "Port $Port is already in use."
    }
}

try {
    Assert-Port-Free -Port $port

    $process = Start-Process -FilePath "cargo.exe" `
        -ArgumentList "run", "--bin", "sensor-health-drift-monitor" `
        -WorkingDirectory $root `
        -RedirectStandardOutput $stdout `
        -RedirectStandardError $stderr `
        -WindowStyle Hidden `
        -PassThru

    Wait-ForUrl "http://127.0.0.1:$port/"

    $edge = Get-EdgePath
    $targets = @(
        @{ Url = "http://127.0.0.1:$port/"; File = "01-overview-proof.png"; Size = "1600,1450" },
        @{ Url = "http://127.0.0.1:$port/sensor-lane"; File = "02-sensor-lane-proof.png"; Size = "1600,1320" },
        @{ Url = "http://127.0.0.1:$port/drift-findings"; File = "03-drift-findings-proof.png"; Size = "1600,1380" },
        @{ Url = "http://127.0.0.1:$port/verification"; File = "04-verification-proof.png"; Size = "1600,1180" }
    )

    foreach ($target in $targets) {
        & $edge `
            --headless `
            --disable-gpu `
            --hide-scrollbars `
            "--window-size=$($target.Size)" `
            "--screenshot=$(Join-Path $screenshots $target.File)" `
            $target.Url | Out-Null
    }
} finally {
    if ($process -and -not $process.HasExited) {
        Stop-Process -Id $process.Id -Force
        $process.WaitForExit()
    }
    foreach ($log in @($stdout, $stderr)) {
        if (Test-Path $log) {
            try {
                Remove-Item $log -Force
            } catch {
                Start-Sleep -Milliseconds 250
                Remove-Item $log -Force -ErrorAction SilentlyContinue
            }
        }
    }
}
