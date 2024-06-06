#!/bin/bash


solana config set --url devnet


output=$(solana-keygen new -o ./wallets/program.json --force)


pubkey=$(echo "$output" | grep -o 'pubkey: .*' | awk '{print $2}')

printf '\n'


echo "use solana_program::declare_id;
pub mod constants;
pub mod entrypoint;
pub mod instructions;
pub mod processor;
pub mod state;
pub mod utils;
declare_id!(\"$pubkey\");
" > "src/lib.rs"


cargo build-sbf


solana program deploy --program-id ./wallets/program.json ./target/deploy/hello_token.so


echo "Successfully deployed program at address: $pubkey"