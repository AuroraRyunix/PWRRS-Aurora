mod wallet;
mod pwr;
mod transaction;
mod block;

use wallet::PWRWallet;
use pwr::PWRRS;
use transaction::{Transaction, TransferTransaction, DataTransaction};
use block::Block;

fn main() {
    // Create instances of PWRWallet and PWRRS as needed
    let private_key = "your_private_key_here"; // Replace with your actual private key
    let wallet = PWRWallet::new(private_key).expect("Failed to create wallet");

    // Use the wallet methods and interact with PWR functionality

    // Create instances of Block, Transaction, TransferTransaction, and DataTransaction
    let block = Block {
        block_hash: "your_block_hash_here".to_string(),
        success: true,
        block_number: 123,
        block_reward: "10 PWR".to_string(),
        transaction_count: 5,
        transactions: vec![
            Transaction::Transfer(TransferTransaction {
                position_in_the_block: 1,
                nonce_or_validation_hash: "nonce_or_validation_hash_here".to_string(),
                size: 256,
                fee: "0.001 PWR".to_string(),
                from: "sender_address_here".to_string(),
                to: "recipient_address_here".to_string(),
                txn_fee: "0.0001 PWR".to_string(),
                type_: "Transfer".to_string(),
                hash: "transaction_hash_here".to_string(),
                value: "5 PWR".to_string(),
            }),
            Transaction::Data(DataTransaction {
                position_in_the_block: 2,
                nonce_or_validation_hash: "nonce_or_validation_hash_here".to_string(),
                size: 512,
                fee: "0.002 PWR".to_string(),
                from: "sender_address_here".to_string(),
                to: "recipient_address_here".to_string(),
                txn_fee: "0.0002 PWR".to_string(),
                type_: "Data".to_string(),
                hash: "transaction_hash_here".to_string(),
                vm_id: "vm_id_here".to_string(),
                data: "data_here".to_string(),
            }),
        ],
        block_submitter: "submitter_address_here".to_string(),
        block_size: 2048,
        timestamp: 1635967200,
    };

    // You can work with the Block and Transaction types here
}
