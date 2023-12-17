use sha2::{Sha256, Digest};
use hex;
use chrono::{DateTime, Local};
use crate::types::{Block, Validator};

pub fn hash_input(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn chrono_timestamp() -> String {
    Local::now().timestamp().to_string()
}

pub fn genesis_block() -> Block{
    let timestamp = chrono_timestamp();
    Block{
        index: 0,
        timestamp: timestamp.clone(),
        bpm: String::from("0"),
        hash: hash_input(&timestamp),
        prev_hash: None,
        validator: Validator{
            address: "0x00".to_string(),
            balance: 0
        }
    }
}

pub fn create_validator_set(n: u64, balances: Vec<u64>) -> Vec<Validator>{
    let mut validators: Vec<Validator> = Vec::new();
    for i in 0..n{
        validators.push(Validator{
            address: format!("validator{}", i),
            balance: balances[i as usize] 
        })
    }
    validators
}