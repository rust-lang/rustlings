# Allows you to change code and see what the rust compiler does immediately in the opposite pane

# Instructions:
# - ./rustle.sh
# - select a rustling, see the split pane view show up
# - update until you have the solution, or you get sick of being stuck..
# - kill the tmux session (Prefix : kill-session)
# - do it all over again

# Built-in niceties:
# - checks you have everything, gives you hints where to get it
# - runs either the 'cargo run' or 'cargo test' based on if the code contains test code or not (naive)
# - override default editor with the EDITOR variable iow:
#     EDITOR="emacs-or-vim -some-switches" ./rustle.sh
# - Should work on anything that has a recent bash (4.3.38 >=)
#
function urldecode() {
    local url_encoded="${1//+/ }"
    result=$(printf '%b' "${url_encoded//%/\\x}")
    echo -e "${result##*code=}"
}

function dependency-check() {
    dependency=$1
    contingency=${2:-""}
    if [ "$(which $dependency)" == "" ]; then
	echo "${dependency} not installed ${contingency}"
	exit 1
    else
	echo "${dependency} is installed!"
    fi
}

function init() {
    # Check for dependencies, create workdir
    declare -A deps
    deps[tmux]="- download tmux for your distro, or compile likaboss. (https://github.com/tmux/tmux)"
    deps[entr]="- download from (http://entrproject.org/)"
    deps[cargo]="- via rustup (https://www.rustup.rs/)"
    deps[rustc]="- via rustup (https://www.rustup.rs/)"
    for dependency in ${!deps[@]}; do
	contingency=${deps[${dependency}]}
	dependency-check ${dependency} "${contingency}"
    done
    WORKDIR=$PWD/rustlings
    if [ -d $WORKDIR ];
    then
	rm -fr $WORKDIR
    fi
    mkdir -p $WORKDIR

    cd $WORKDIR
}



function extract-code-from-markdown() {
    # extracts the rust code from the markdown url via urldecode function
    grep -oP '\["[^"]+"\]\([^\)]+' ../../README.md | sed 's/["()[]//g'| sed 's/[]]/ /g' | while read -a matches;
    do
	filename=${matches[0]}
	rustling=${filename%%.rs}
	cargo new ${rustling} --bin
	echo "$(urldecode ${matches[1]})" > ${rustling}/src/main.rs
    done
}


function run-selection() {
    selection=${1}
    SESSION_NAME=${2}

    tmux new-session -s ${SESSION_NAME} -d
    tmux new-window -t ${SESSION_NAME}:1
    tmux split-window -t ${SESSION_NAME}:1 -h
    if [ "$(grep -R '\[test\]' ${selection}/src/*rs)" == "" ]; then
	tmux send-keys -t ${SESSION_NAME}:1.1 "cd ${selection} && ls ./src/*rs |entr bash -c 'clear && cargo run'" C-m
    else
	tmux send-keys -t ${SESSION_NAME}:1.1 "cd ${selection} && ls ./src/*rs |entr bash -c 'clear && cargo test'" C-m
    fi
    tmux send-keys -t ${SESSION_NAME}:1.0 "${EDITOR} ${selection}/src/main.rs; ${QUIT_SESSION}" C-m
    tmux select-pane -t ${SESSION_NAME}:1.0
    tmux attach -t ${SESSION_NAME}    
}

function menu-selection() {
    # Generate a menu
    WORKDIR=${1}
    EDITOR=${2:-nano}
    PS3='Please enter your choice: '
    declare -A options
    for i in `ls -d ${WORKDIR}/*`;
    do
	key=${i##*/}
	options[$key]=$(realpath $i);
    done
    
    
    keys=$(ls -d ${!options[@]})
    select opt in $keys
    do
	selection=${options[$opt]}
	echo "${opt}..${selection}"
	run-selection $selection rustalisk # rustalisk is a working title, I'm a starcraft fan. (Zerglings? Rustlings? Hydralisks...")
    done
}



init;
extract-code-from-markdown;
menu-selection $WORKDIR;
