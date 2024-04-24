mod utils;
use std::{ops::Mul, str::FromStr};

use mv3_contract_solana::state::config::MessageClient;
use solana_program::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey};
use solana_sdk::{signer::Signer, transaction::Transaction};

use crate::utils::{get_authority, get_rpc};

pub fn main() {
    let authority = get_authority();

    let args: Vec<String> = std::env::args().collect();

    let network = args.get(1).expect("Missing network!");

    let destination = args.get(2).expect("Missing contract!");

    let parsed_destination = Pubkey::from_str(&destination);

    if parsed_destination.is_err() {
        println!("Contract address is not valid Solana public key!");
        return;
    }

    let raw_amount = args.get(3).expect("Missing deposit amount!");

    let parsed_amount = f32::from_str(&raw_amount);

    if parsed_amount.is_err() {
        println!("Invalid initial deposit amount!");
        return;
    }

    let rpc = get_rpc(network.clone());

    let destination_contract = parsed_destination.unwrap();

    let deposit_ix = mv3_contract_solana::instruction::deposit_withdraw_sol(
        authority.pubkey(),
        destination_contract,
        (parsed_amount.unwrap().mul(LAMPORTS_PER_SOL as f32)) as u64,
        mv3_contract_solana::instruction::DepositWithdraw::Deposit,
    );

    let ix = mv3_contract_solana::instruction::configure_client(
        authority.pubkey(),
        MessageClient {
            allowed_contracts: vec![],
            authority: authority.pubkey(),
            destination_contract,
            exsig: None,
            notify_on_failure: false,
            supported_chains: vec![80001],
        },
    );

    let mut tx = Transaction::new_with_payer(&[ix, deposit_ix], Some(&authority.pubkey()));

    tx.sign(&[&authority], rpc.get_latest_blockhash().unwrap());

    let signature = rpc.send_and_confirm_transaction(&tx).unwrap();

    println!("Configured client with hash: {:?}", signature.to_string());
}
