use axios::Axios;
use hex;
use secp256k1::key::SecretKey;
use secp256k1::Secp256k1;
use std::convert::TryFrom;
use std::error::Error;
use std::result::Result;
use utils::{BnToBytes, decToBytes, bytesToHex};
use web3::signing::keccak256;
use web3::signing::keccak256::keccak256;

mod utils;

pub struct PWRWallet {
    address: String,
    private_key: String,
    url: &'static str,
}

impl PWRWallet {
    pub fn new(private_key: String) -> Result<Self, Box<dyn Error>> {
        let wallet = WalletUtils::from_private_key(&private_key)?;

        let address = wallet.get_address_string();

        Ok(PWRWallet {
            address,
            private_key,
            url: "https://pwrrpc.pwrlabs.io",
        })
    }

    pub fn get_address(&self) -> String {
        self.address.clone()
    }

    pub async fn get_balance(&self) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/balanceOf/?userAddress={}", self.url, self.address);
        let response = axios::get(&url).await?;
        let data = response.data();
        if data["status"] == "success" {
            Ok(data["data"]["balance"].to_string())
        } else {
            Err("Error getting balance".into())
        }
    }

    pub async fn get_nonce(&self) -> Result<i32, Box<dyn Error>> {
        let url = format!("{}/nonceOfUser/?userAddress={}", self.url, self.address);
        let response = axios::get(&url).await?;
        let data = response.data();
        if data["status"] == "success" {
            Ok(data["data"]["nonce"].as_i32().unwrap())
        } else {
            Err("Error getting nonce".into())
        }
    }

    pub fn get_private_key(&self) -> String {
        self.private_key.clone()
    }

    pub async fn transfer_pwr(&self, to: &str, amount: &str, nonce: Option<i32>) -> Result<String, Box<dyn Error>> {
        let id = 0;
        let nonce = nonce.unwrap_or(self.get_nonce().await?);

        let txn_data_bytes = generate_txn_bytes(id, nonce, amount, to);
        let signed_txn_bytes = sign_txn(&txn_data_bytes, &self.private_key)?;

        let mut txn_bytes = vec![];
        txn_bytes.extend_from_slice(&txn_data_bytes);
        txn_bytes.extend_from_slice(&signed_txn_bytes);

        let txn_hex = hex::encode(&txn_bytes);

        let url = format!("{}/broadcast/", self.url);
        let response = axios::post(&url, &json!({ "txn": txn_hex })).await?;
        let data = response.data();
        if data["status"] == "success" {
            Ok(data["data"].to_string())
        } else {
            Err("Error sending transaction".into())
        }
    }

    pub async fn send_vm_data_txn(&self, vm_id: &str, data_bytes: &[u8], nonce: Option<i32>) -> Result<String, Box<dyn Error>> {
        let id = 5;
        let nonce = nonce.unwrap_or(self.get_nonce().await?);
        let _vm_id = vm_id;
        let data = bytes_to_hex(data_bytes);

        let txn_data_bytes = generate_data_txn_bytes(id, nonce, _vm_id, &data);
        let signed_txn_bytes = sign_txn(&txn_data_bytes, &self.private_key)?;

        let mut txn_bytes = vec![];
        txn_bytes.extend_from_slice(&txn_data_bytes);
        txn_bytes.extend_from_slice(&signed_txn_bytes);

        let txn_hex = hex::encode(&txn_bytes);

        let url = format!("{}/broadcast/", self.url);
        let response = axios::post(&url, &json!({ "txn": txn_hex })).await?;
        let data = response.data();
        if data["status"] == "success" {
            Ok(data["data"].to_string())
        } else {
            Err("Error sending transaction".into())
        }
    }
}

fn generate_txn_bytes(id: i32, nonce: i32, amount: &str, recipient_sr: &str) -> Vec<u8> {
    let id_byte = dec_to_bytes(id, 1);
    let nonce_byte = dec_to_bytes(nonce, 4);
    let amount_bn = BnToBytes::try_from(amount)?;
    let recipient = recipient_sr.trim_start_matches("0x");
    let recipient_byte = hex::decode(recipient).unwrap();

    let mut txn_bytes = vec![];
    txn_bytes.extend_from_slice(&id_byte);
    txn_bytes.extend_from_slice(&nonce_byte);
    txn_bytes.extend_from_slice(&amount_bn);
    txn_bytes.extend_from_slice(&recipient_byte);

    txn_bytes
}

fn generate_data_txn_bytes(id: i32, nonce: i32, vm_id: &str, data: &str) -> Vec<u8> {
    let id_byte = dec_to_bytes(id, 1);
    let nonce_byte = dec_to_bytes(nonce, 4);
    let vm_id_bn = BnToBytes::try_from(vm_id)?;
    let data_bytes = hex::decode(data).unwrap();

    let mut txn_bytes = vec![];
    txn_bytes.extend_from_slice(&id_byte);
    txn_bytes.extend_from_slice(&nonce_byte);
    txn_bytes.extend_from_slice(&vm_id_bn);
    txn_bytes.extend_from_slice(&data_bytes);

    txn_bytes
}

fn sign_txn(txn_bytes: &[u8], private_key: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let hashed_bytes = keccak256(txn_bytes);
    let private_key_bytes = hex::decode(private_key.trim_start_matches("0x"))?;
    let secp = Secp256k1::new();

    let secret_key = SecretKey::from_slice(&secp, &private_key_bytes)?;
    let message = web3::signing::keccak256::keccak256(&hashed_bytes);

    let signature = secp.sign(&message, &secret_key);

    Ok(signature.serialize_compact().to_vec())
}
