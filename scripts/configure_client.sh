#!/bin/bash


network=""
contract=""
amount=""


while [[ $# -gt 0 ]]; do
    case "$1" in
        --network)
            shift
            network="$1"
            ;;
        --contract)
            shift
            contract="$1"
            ;;
        --amount)
            shift
            amount="$1"
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
    shift
done



if [[ -z "$network" ]]; then
    echo "Error: Missing network argument!"
    exit 1
fi

if [[ -z "$contract" ]]; then
    echo "Error: Missing contract"
    exit 1
fi

if [[ -z "$amount" ]]; then
    echo "Error: Missing deposit amount"
    exit 1
fi

cargo run --bin configure_client $network $contract $amount
