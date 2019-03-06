#!/usr/bin/env bash

echo "Let's get you set up with Rustlings!"

echo "Checking requirements..."
if [ -x "$(git)" ]
then
    echo "WARNING: Git does not seem to be installed."
    echo "Please download Git using your package manager or over https://git-scm.com/!"
    exit 1
else
    echo "SUCCESS: Git is installed"
fi

if [ -x "$(rustc)" ]
then
    echo "WARNING: Rust does not seem to be installed."
    echo "Please download Rust using https://rustup.rs!"
    exit 1
else
    echo "SUCCESS: Rust is installed"
fi

if [ -x "$(cargo)" ]
then
    echo "WARNING: Cargo does not seem to be installed."
    echo "Please download Rust and Cargo using https://rustup.rs!"
    exit 1
else
    echo "SUCCESS: Cargo is installed"
fi

Path=${1:-rustlings/}
echo "Cloning Rustlings at $Path..."
git clone -q https://github.com/rust-lang/rustlings $Path

Version=$(curl -s https://api.github.com/repos/rust-lang/rustlings/releases/latest | python -c "import json,sys;obj=json.load(sys.stdin);print(obj['tag_name']);")
echo "Checking out version $Version..."
cd $Path
git checkout -q tags/$Version

echo "Installing the 'rustlings' executable..."
cargo install --force --path .

if [ -x "$(rustlings)" ]
then
    echo "WARNING: Please check that you have '~/.cargo/bin' in your PATH environment variable!"
fi

echo "All done! Run 'rustlings' to get started."

