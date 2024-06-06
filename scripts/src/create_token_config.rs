use std::str::FromStr;
mod utils;
use crate::utils::{get_authority, get_rpc};
use ethers::core::rand::thread_rng;
use hello_token::instructions::{create_token, init_extra_account_meta_list};
use message_hook::get_extra_account_metas_address;
use mv3_contract_solana::constants::MESSAGE_CLIENT_SEED;
use solana_sdk::{
    program_pack::Pack, pubkey::Pubkey, rent::Rent, signature::Keypair, signer::Signer,
    system_instruction, system_program, transaction::Transaction,
};
use spl_tlv_account_resolution::{
    account::ExtraAccountMeta, seeds::Seed, state::ExtraAccountMetaList,
};

pub fn main() {
    let args: Vec<String> = std::env::args().collect();

    let network = args.get(1).expect("Missing network config!");

    let name = args.get(2).expect("Missing token name!");

    let symbol = args.get(3).expect("Missing token symbol!");

    let supply = args.get(4).expect("Missing token supply!");

    let parsed_supply = u64::from_str(supply);

    if parsed_supply.is_err() {
        println!("Invalid token supply!");
        return;
    }

    let rpc = get_rpc(network.clone());

    let mint_authority = get_authority();

    let mint = Keypair::generate(&mut thread_rng());

    let create_token_ix = create_token(
        name.to_string(),
        symbol.to_string(),
        parsed_supply.unwrap(),
        9,
        mint_authority.pubkey(),
        mint.pubkey(),
        hello_token::id(),
    );

    let addr = Pubkey::find_program_address(
        &[b"token", mint_authority.pubkey().as_ref()],
        &hello_token::id(),
    );

    let (message_pda, _) = Pubkey::find_program_address(
        &[MESSAGE_CLIENT_SEED, hello_token::id().as_ref()],
        &mv3_contract_solana::id(),
    );

    let token = Keypair::generate(&mut thread_rng());

    let create_account = system_instruction::create_account(
        &mint_authority.pubkey(),
        &token.pubkey(),
        Rent::default().minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN as u64,
        &spl_token::id(),
    );

    let extra_account_metas: Vec<ExtraAccountMeta> = vec![
        ExtraAccountMeta::new_with_pubkey(&addr.0, false, true).unwrap(),
        ExtraAccountMeta::new_with_pubkey(&system_program::id(), false, false).unwrap(),
        ExtraAccountMeta::new_with_pubkey(
            &Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(),
            false,
            true,
        )
        .unwrap(),
        ExtraAccountMeta::new_with_pubkey(
            &Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL").unwrap(),
            false,
            false,
        )
        .unwrap(),
        ExtraAccountMeta::new_with_pubkey(&mint.pubkey(), false, true).unwrap(),
        ExtraAccountMeta::new_external_pda_with_seeds(
            2,
            &[
                Seed::InstructionData {
                    index: 0,
                    length: 32,
                },
                Seed::AccountKey { index: 2 },
                Seed::AccountKey { index: 4 },
            ],
            false,
            true,
        )
        .unwrap(),
    ];

    let rent_lamports = Rent::default()
        .minimum_balance(ExtraAccountMetaList::size_of(extra_account_metas.len()).unwrap());

    let init_extra_account_meta = init_extra_account_meta_list(
        hello_token::id(),
        mint_authority.pubkey(),
        extra_account_metas.clone(),
    );

    let extra_account_metas_address =
        get_extra_account_metas_address(&message_pda, &hello_token::id());

    let mut transaction = Transaction::new_with_payer(
        &[
            create_account,
            create_token_ix,
            system_instruction::transfer(
                &mint_authority.pubkey(),
                &extra_account_metas_address,
                rent_lamports,
            ),
            init_extra_account_meta,
        ],
        Some(&mint_authority.pubkey()),
    );

    transaction.sign(
        &[&mint_authority, &token, &mint],
        rpc.get_latest_blockhash().unwrap(),
    );

    let hash = rpc.send_and_confirm_transaction(&transaction).unwrap();

    println!("Initialized token config: {:?}", hash.to_string());
}
