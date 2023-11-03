pub struct Transaction {
    pub position_in_the_block: i32,
    pub nonce_or_validation_hash: String,
    pub size: i32,
    pub fee: String,
    pub from: String,
    pub to: String,
    pub txn_fee: String,
    pub type_: String, // Using "type_" instead of "type" to avoid keyword conflict
    pub hash: String,
}

pub struct TransferTransaction {
    pub value: String,
    // Include fields from the Transaction struct
    pub position_in_the_block: i32,
    pub nonce_or_validation_hash: String,
    pub size: i32,
    pub fee: String,
    pub from: String,
    pub to: String,
    pub txn_fee: String,
    pub type_: String,
    pub hash: String,
}

pub struct DataTransaction {
    pub vm_id: String,
    pub data: String,
    // Include fields from the Transaction struct
    pub position_in_the_block: i32,
    pub nonce_or_validation_hash: String,
    pub size: i32,
    pub fee: String,
    pub from: String,
    pub to: String,
    pub txn_fee: String,
    pub type_: String,
    pub hash: String,
}
