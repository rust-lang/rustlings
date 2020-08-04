#!/usr/bin/env bash

# Compare two version strings, v1 and v2, passed as arguments (e.g 1.31 and
# 1.33.0). Returns 1 if v1 > v2, 0 if v1 == v2, 2 if v1 < v2.
compare_versions() {
    [ "$1" == "$2" ] && return 0
    declare -a v1=() v2=()
    mapfile -d . -t v1 < <(echo "$1")
    mapfile -d . -t v2 < <(echo "$2")
    declare -ir max_len=$(( ${#v1[@]} > ${#v2[@]} ? ${#v1[@]} : ${#v2[@]} ))
    for i in $(seq 0 "$max_len")
    do
        # Fill empty fields with zeros.
        [ -z "${v1[$i]}" ] && v1["$i"]=0
        [ -z "${v2[$i]}" ] && v2["$i"]=0
        # Compare the fields.
        [ ${v1[$i]} -gt ${v2[$i]} ] && return 1
        [ ${v1[$i]} -lt ${v2[$i]} ] && return 2
    done
    return 0
}

# Return 0 if the argument represents a valid command on the machine, 1
# otherwise.
verify_dependency() {
    declare -r package="$1"
    if [ -x "$(command -v "$package")" ]
    then
        echo "SUCCESS: \`$package\` is installed."
        return 0
    else
        echo "ERROR: \`$package\` does not seem to be installed." >&2
        return 1
    fi
}

# Main logic begins here
declare Clippy Py RustVersion Version
declare -r CargoBin="${CARGO_HOME:-$HOME/.cargo}/bin"

echo "Let's get you set up with Rustlings!"

# Verify that the required dependencies are installed.
echo "Checking requirements..."
for dependency in cargo cc curl git rustc
do
    verify_dependency "$dependency" || exit 1
done
# Search for a Python installation; start with 3 and fallback to 2 if required.
if [ -x "$(command -v python3)" ]
then
    Py="$(command -v python3)"
elif [ -x "$(command -v python)" ]
then
    Py="$(command -v python)"
elif [ -x "$(command -v python2)" ]
then
    Py="$(command -v python2)"
else
    echo "ERROR: no working Python installation was found." >&2
    echo "Please install Python and add it to the PATH variable." >&2
    exit 1
fi

# Ensure the installed Rust compiler is sufficiently up-to-date.
RustVersion=$(rustc --version | cut -d " " -f 2)
compare_versions "$RustVersion" "$MinRustVersion"
if [ $? -eq 2 ]
then
    echo "ERROR: Rust $RustVersion is too old; the minimum version is" \
         "$MinRustVersion." >&2
    echo "Please update Rust with \`rustup update\`." >&2
    exit 1
else
    echo "SUCCESS: Rust is up to date."
fi

# Clone the Git repository.
echo "Cloning Rustlings into '$Path'..."
if git clone -q "https://github.com/rust-lang/rustlings" "$Path"
then
    cd "$Path" || exit 1
else
    echo "ERROR: failed to clone the Git repository into '$Path'." >&2
    exit 1
fi

# Determine which version of Rustlings to install from the Git repository.
Version=$(
    curl -s https://api.github.com/repos/rust-lang/rustlings/releases/latest \
    | "$Py" -c \
            "import json,sys;obj=json.load(sys.stdin);print(obj['tag_name'])"
)
if [ -z "$Version" ]
then
    echo "The latest tag version could not be fetched remotely, using the" \
         "local Git repository..."
    Version=$(ls -t .git/refs/tags/ | head --lines=1)
    if [ -z "$Version" ]
    then
        echo "No valid tag version was found; Rustlings will be installed" \
             "from the main Git branch."
        Version="main"
    else
        Version="tags/$Version"
    fi
else
    Version="tags/$Version"
fi

# Install the selected version of Rustlings.
echo "Checking out version $Version..."
git checkout -q "$Version"
echo "Installing the \`rustlings\` executable..."
cargo install --force --path .

# Verify that the Rustlings installation was successful.
if ! [ -x "$(command -v rustlings)" ]
then
    echo "WARNING: Please ensure '$CargoBin' is in your PATH environment" \
         "variable!"
fi

# Ensure Clippy is installed (due to a bug in Cargo, this must be done with
# Rustup: https://github.com/rust-lang/rustup/issues/1514).
Clippy=$(rustup component list | grep 'clippy.*(installed)')
if [ -z "$Clippy" ]
then
    echo "Installing the \`cargo-clippy\` executable..."
    rustup component add clippy
fi

echo "All done! Run \`rustlings\` to get started."
