#!/bin/bash


network=""
name=""
symbol=""
supply=""


while [[ $# -gt 0 ]]; do
    case "$1" in
        --network)
            shift
            network="$1"
            ;;
        --name)
            shift
            name="$1"
            ;;
        --symbol)
            shift
            symbol="$1"
            ;;
        --supply)
            shift
            supply="$1"
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

if [[ -z "$name" ]]; then
    echo "Error: Missing token name"
    exit 1
fi

if [[ -z "$symbol" ]]; then
    echo "Error: Missing token symbol"
    exit 1
fi

if [[ -z "$supply" ]]; then
    echo "Error: Missing token supply"
    exit 1
fi

cargo run --bin create_token_config $network $name $symbol $supply
