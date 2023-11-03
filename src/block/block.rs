use crate::transaction::{DataTransaction, TransferTransaction};

pub struct Block {
    pub block_hash: String,
    pub success: bool,
    pub block_number: i32,
    pub block_reward: String,
    pub transaction_count: i32,
    pub transactions: Vec<Transaction>,
    pub block_submitter: String,
    pub block_size: i32,
    pub timestamp: i64,
}

// Define the Transaction enum to represent both TransferTransaction and DataTransaction
pub enum Transaction {
    Transfer(TransferTransaction),
    Data(DataTransaction),
}
