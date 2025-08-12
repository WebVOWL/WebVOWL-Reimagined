#!/bin/bash

modes=("dev" "preview" "release")
commands=(
    "cargo leptos watch --wasm-debug -v"
    "cargo leptos watch --release --precompress --wasm-debug -v"
    "cargo leptos build --release --precompress -v"
    )
help=(
    "Builds WebVOWL in development mode and runs it on a local server"
    "Builds WebVOWL in production mode but with debug info and runs it on a local server"
    "Builds WebVOWL in production mode, ready for deployment"
    )

valid=0
#1 Compares first input argument in argv against available modes.
for ((i=0; i < ${#modes[@]}; i++)); do
    if [[ $1 = ${modes[i]} ]]; then
        # Execute the command associated with the chosen mode
        bash -c "${commands[i]}"
        valid=1
    fi
done

#2 Checks if #1 found a valid input argument. If not, show help message.
if [[ $valid -ne 1 ]]; then
    help_string=""
    for ((i=0; i < ${#modes[@]}; i++)); do
        help_string+="    "
        help_string+=${modes[i]}
        help_string+="        "
        help_string+=${help[i]}
        help_string+=$'\n'
    done

    echo "Invalid argument '$1'. Expected one of:"
    echo "$help_string"
fi