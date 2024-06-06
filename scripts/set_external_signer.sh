#!/bin/bash


network=""
contract=""
evm_address=""


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
        --evm_address)
            shift
            evm_address="$1"
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

if [[ -z "$evm_address" ]]; then
    echo "Error: Missing EVM address"
    exit 1
fi

cargo run --bin set_exsig $network $contract $evm_address
