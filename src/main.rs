use solana_client::rpc_client::RpcClient;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

struct Transaction {
    sender: String,
    receiver: String,
    amount: f64,
    slot: u64, // Using slot instead of timestamp
}

fn check_solana_transactions(
    target_receiver_account: &str,
    beginning_block_number: u64,
    ending_block_number: u64,
    transactions: &[Transaction],
    presale_balances: &mut HashMap<String, u64>,
    presale_allocations: &mut HashMap<String, u64>,
) {
    let mut total_raised_amount = 0.0;
    let mut unique_accounts = HashSet::new();
    let presale_amount: u64 = 774_000_000;

    for transaction in transactions {
        if transaction.receiver == target_receiver_account
            && transaction.slot >= beginning_block_number
            && transaction.slot <= ending_block_number
            && transaction.amount >= 0.1
        {
            total_raised_amount += transaction.amount;
            unique_accounts.insert(&transaction.sender);
            *presale_balances.entry(transaction.sender.clone()).or_insert(0) += transaction.amount as u64;
        }
    }

    for (account, amount) in presale_balances.iter() {
        let presale_share = (*amount as f64 * 100.0) / total_raised_amount;
        let alloc = (presale_amount as f64 * presale_share) as u64 / 100;
        *presale_allocations.entry(account.clone()).or_insert(0) += alloc;
    }
}

fn fetch_solana_transactions(
    target_receiver_account: &str,
    beginning_block_number: u64,
    ending_block_number: u64,
) -> Vec<Transaction> {
    let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
    let transactions = rpc_client.get_transactions(target_receiver_account, ending_block_number - beginning_block_number, Some(beginning_block_number)).unwrap();
    
    transactions.into_iter().map(|tx| {
        Transaction {
            sender: tx.sender.to_string(),
            receiver: tx.receiver.to_string(),
            amount: tx.amount,
            slot: tx.slot,
        }
    }).collect()
}

fn fetch_block_number() -> u64 {
    let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
    let block_info = rpc_client.get_block_production().unwrap();
    block_info.block_height
}

fn save_to_json_file(file_path: &str, data: &HashMap<String, u64>) -> std::io::Result<()> {
    let json_data = serde_json::to_string(data)?;

    let mut file = File::create(file_path)?;
    file.write_all(json_data.as_bytes())?;

    Ok(())
}

fn main() {
    let target_receiver_account = "EWFNcP9W8RiaUZPqa6baaiZR3wDoPxy1N3g8dAhrnSFe";
    let beginning_block_number = 260669993; // April 17, 2024 14:34:30 UTC

    // Fetch current block number
    let ending_block_number = fetch_block_number();

    let transactions = fetch_solana_transactions(target_receiver_account, beginning_block_number, ending_block_number);
    
    let mut presale_balances = HashMap::new();
    let mut presale_allocations = HashMap::new();

    let balances_file_path = "../dist/presale_balances.json";
    let allocations_file_path = "../dist/presale_allocations.json";

    save_to_json_file(balances_file_path, &presale_balances).unwrap();
    save_to_json_file(allocations_file_path, &presale_allocations).unwrap();

    check_solana_transactions(target_receiver_account, beginning_block_number, ending_block_number, &transactions, &mut presale_balances, &mut presale_allocations);
}
