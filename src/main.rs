//! This is a part of 258 Bot
//
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair},
    transaction::Transaction,
};
use spl_token::instruction::transfer;
use std::{fs, str::FromStr, error::Error};
use csv::ReaderBuilder;

fn make_airdrop(csv_file: &str) -> Result<(), Box<dyn Error>> {
    // Load the sender's keypair path from .env file
    dotenv::dotenv().ok();
    let sender_keypair_path = std::env::var("SENDER_KEYPAIR_PATH")?;
    let sender_keypair = read_keypair_file(&sender_keypair_path)?;
    let sender_pubkey = sender_keypair.pubkey();

    // Connect to Solana cluster
    let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com");

    // Token details
    let token_mint = Pubkey::from_str("TokenMintAddressHere")?;
    let sender_token_account = Pubkey::from_str("SenderTokenAccountAddressHere")?;

    // Read account details from the CSV file
    let file = fs::File::open(csv_file)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);

    // Initialize variables for batching
    let mut transfer_instructions = vec![];
    let batch_size = 1000; // Number of transfers to include in each batch

    for result in rdr.records() {
        let record = result?;
        let recipient_pubkey = Pubkey::from_str(&record[0])?;
        let amount: u64 = record[1].parse()?;

        let recipient_token_account = derive_token_account_address(&recipient_pubkey, &token_mint);

        let transfer_instruction = transfer(
            &spl_token::id(),
            &sender_token_account,
            &recipient_token_account,
            &sender_pubkey,
            &[&sender_pubkey],
            amount,
        )?;

        transfer_instructions.push(transfer_instruction);

        // Check if the batch size is reached
        if transfer_instructions.len() == batch_size {
            // Create and sign transaction
            let recent_blockhash = rpc_client.get_recent_blockhash()?.0;
            let transaction = Transaction::new_signed_with_payer(
                &transfer_instructions,
                Some(&sender_pubkey),
                &[&sender_keypair],
                recent_blockhash,
            );

            // Send transaction
            let result = rpc_client.send_and_confirm_transaction(&transaction);
            println!("Batch Transaction result: {:?}", result);

            // Clear the transfer instructions for the next batch
            transfer_instructions.clear();
        }
    }

    // Process any remaining transfers
    if !transfer_instructions.is_empty() {
        let recent_blockhash = rpc_client.get_recent_blockhash()?.0;
        let transaction = Transaction::new_signed_with_payer(
            &transfer_instructions,
            Some(&sender_pubkey),
            &[&sender_keypair],
            recent_blockhash,
        );

        let result = rpc_client.send_and_confirm_transaction(&transaction);
        println!("Batch Transaction result: {:?}", result);
    }

    Ok(())
}

// Function to derive or find the recipient's token account address
fn derive_token_account_address(recipient: &Pubkey, token_mint: &Pubkey) -> Pubkey {
    // Placeholder logic to derive or find the recipient's token account address
    *recipient // Replace with actual logic
}

fn main() {
    if let Err(err) = make_airdrop("accounts.csv") {
        eprintln!("Error: {}", err);
    }
}
