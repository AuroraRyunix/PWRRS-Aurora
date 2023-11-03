use reqwest;
use std::error::Error;
use serde_json::Value;
use tokio;

pub struct PWRRS;

impl PWRRS {
    static rpc_node_url: Option<String> = None;
    static fee_per_byte: u32 = 100;

    pub fn get_rpc_node_url() -> Result<String, Box<dyn Error>> {
        match &PWRRS::rpc_node_url {
            Some(url) => Ok(url.clone()),
            None => Err("RPC Node URL is not defined".into()),
        }
    }

    pub async fn get_nonce_of_address(address: &str) -> Result<String, Box<dyn Error>> {
        let url = format!(
            "{}/nonceOfUser/?userAddress={}",
            PWRRS::get_rpc_node_url()?,
            address
        );

        let res = reqwest::get(&url).await?;

        let response = res.json::<Value>().await?;

        if response["status"] != "success" {
            return Err("Error getting nonce".into());
        }

        Ok(response["data"]["nonce"].to_string())
    }

    pub async fn get_balance_of_address(address: &str) -> Result<String, Box<dyn Error>> {
        let url = format!(
            "{}/balanceOf/?userAddress={}",
            PWRRS::get_rpc_node_url()?,
            address
        );

        let res = reqwest::get(&url).await?;

        let response = res.json::<Value>().await?;

        if response["status"] != "success" {
            return Err("Error getting balance".into());
        }

        Ok(response["data"]["balance"].to_string())
    }

    pub fn get_fee_per_byte() -> u32 {
        PWRRS::fee_per_byte
    }

    pub async fn get_blocks_count() -> Result<u32, Box<dyn Error>> {
        let url = format!("{}/blocksCount/", PWRRS::get_rpc_node_url()?);

        let res = reqwest::get(&url).await?;

        let response = res.json::<Value>().await?;

        if response["status"] != "success" {
            return Err("Error getting balance".into());
        }

        Ok(response

