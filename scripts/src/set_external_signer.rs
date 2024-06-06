use borsh::ser::BorshSerialize;
use mv3_contract_solana::{instruction::V3Instruction, utils::get_message_client_pda};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};
use solana_sdk::transaction::Transaction;
use solana_sdk::{commitment_config::CommitmentConfig, signer::Signer};
use std::str::FromStr;
use utils::{get_authority, get_rpc};

mod utils;

pub fn main() {
    let args: Vec<String> = std::env::args().collect();

    println!("ARGS {:?}", args);

    let network = args.get(1).expect("Missing network config!");

    let rpc = get_rpc(network.to_string());

    let authority = get_authority();

    let contract = args.get(2).expect("Missing solana contract!");

    let contract_address = Pubkey::from_str(&contract);

    match contract_address {
        Ok(_) => {}
        Err(_) => {
            println!("Invalid Solana contract address!");
            return;
        }
    }

    let evm_address = args.get(3).expect("Missing EVM chainsig address!");

    if !evm_address.starts_with("0x") {
        println!("Invalid EVM address!");
        return;
    }

    let mut chars: Vec<char> = evm_address.chars().collect();

    let _: Vec<char> = chars.splice(0..2, std::iter::empty()).collect();

    let formatted_address: String = chars.into_iter().collect();

    let encoded = hex::decode(formatted_address).expect("Could not serialize address!");

    if encoded.len() != 20 {
        println!("EVM address does not have 20 bytes!");
        return;
    }

    let mut encoded_chainsig: Vec<u8> = vec![];
    encoded_chainsig.extend_from_slice(&[0; 12]);
    encoded_chainsig.extend_from_slice(&encoded);

    let (pda, _) = get_message_client_pda(contract_address.unwrap());

    let accounts: Vec<AccountMeta> = vec![
        AccountMeta {
            is_signer: true,
            is_writable: true,
            pubkey: authority.pubkey(),
        },
        AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: pda,
        },
        AccountMeta {
            is_signer: false,
            is_writable: false,
            pubkey: system_program::id(),
        },
    ];

    let data = V3Instruction::SetExsig {
        exsig: encoded_chainsig.clone().try_into().unwrap(),
    }
    .try_to_vec()
    .unwrap();

    println!("ASDF: {:?}", encoded_chainsig.clone());

    let ix = Instruction {
        program_id: mv3_contract_solana::id(),
        accounts: accounts,
        data,
    };

    let mut tx = Transaction::new_with_payer(&[ix], Some(&authority.pubkey()));

    tx.sign(&[&authority], rpc.get_latest_blockhash().unwrap());

    let sig = rpc
        .send_and_confirm_transaction_with_spinner_and_commitment(
            &tx,
            CommitmentConfig::confirmed(),
        )
        .unwrap();

    println!("Executed tx: {}", sig.to_string());
}
