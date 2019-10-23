#!/usr/bin/env pwsh

#Requires -Version 5
param($path = "$pwd/rustlings")

Write-Host "Let's get you set up with Rustlings!"

Write-Host "Checking requirements..."
if (Get-Command git -ErrorAction SilentlyContinue) {
    Write-Host "SUCCESS: Git is installed"
} else {
    Write-Host "WARNING: Git does not seem to be installed."
    Write-Host "Please download Git using your package manager or over https://git-scm.com/!"
    exit 1
}

if (Get-Command rustc -ErrorAction SilentlyContinue) {
    Write-Host "SUCCESS:  Rust is installed"
} else {
    Write-Host "WARNING: Rust does not seem to be installed."
    Write-Host "Please download Rust using https://rustup.rs!"
    exit 1
}

if (Get-Command cargo -ErrorAction SilentlyContinue) {
    Write-Host "SUCCESS: Cargo is installed"
} else {
    Write-Host "WARNING: Cargo does not seem to be installed."
    Write-Host "Please download Rust and Cargo using https://rustup.rs!"
    exit 1
}

# Function that compares two versions strings v1 and v2 given in arguments (e.g 1.31 and 1.33.0).
# Returns 1 if v1 > v2, 0 if v1 == v2, 2 if v1 < v2.
function vercomp($v1, $v2) {
    if ($v1 -eq $v2) {
       return 0
    } 

    $v1 = $v1.Replace(".", "0")
    $v2 = $v2.Replace(".", "0")
    if ($v1.Length -gt $v2.Length) {
        $v2 = $v2.PadRight($v1.Length, "0")
    } else {
        $v1 = $v1.PadRight($v2.Length, "0")
    }

    if ($v1 -gt $v2) {
        return 1
    } else {
        return 2
    }
}

$rustVersion = $(rustc --version).Split(" ")[1]
$minRustVersion = "1.31"
if ((vercomp $rustVersion $minRustVersion) -eq 2) {
    Write-Host "WARNING: Rust version is too old: $rustVersion - needs at least $minRustVersion"
    Write-Host "Please update Rust with 'rustup update'"
    exit 1
} else {
    Write-Host "SUCCESS: Rust is up to date"
}

Write-Host "Cloning Rustlings at $path"
git clone -q https://github.com/rust-lang/rustlings $path
if (!($LASTEXITCODE -eq 0)) {
    exit 1
}

# UseBasicParsing is deprecated, pwsh 6 or above will automatically use it,
# but anyone running pwsh 5 will have to pass the argument.
$version = Invoke-WebRequest -UseBasicParsing https://api.github.com/repos/rust-lang/rustlings/releases/latest `
    | ConvertFrom-Json | Select-Object -ExpandProperty tag_name
Write-Host "Checking out version $version..."
Set-Location $path
git checkout -q tags/$version

Write-Host "Installing the 'rustlings' executable..."
cargo install --force --path .
if (!(Get-Command rustlings -ErrorAction SilentlyContinue)) {
    Write-Host "WARNING: Please check that you have '~/.cargo/bin' in your PATH environment variable!"
}

Write-Host "All done! Run 'rustlings' to get started."
