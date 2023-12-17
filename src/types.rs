use serde::{Serialize, Deserialize};
use serde_json;
use crate::helpers::{hash_input, chrono_timestamp};

#[derive(Serialize, Deserialize, Clone)]
pub struct Block{
    pub index: u64,
    pub timestamp: String,
    pub bpm: String,
    pub hash: String,
    // None if genesis block
    pub prev_hash: Option<String>,
    pub validator: Validator
}

impl Block{
    pub fn block_hash_from_instance(&self) -> String {
        hash_input(&format!("{}{}{}{}", &self.index, &self.timestamp, &self.bpm, &self.prev_hash.as_ref().unwrap()))
    }
    pub fn block_hash_from_params(index: u64, timestamp: &str, bpm: String, prev_hash: Option<&String>) -> String {
        hash_input(&format!("{}{}{}{}", index, timestamp, bpm, prev_hash.unwrap_or(&"genesis".to_string())))
    }
    pub fn new(index: u64, timestamp: &str, bpm: String, prev_hash: Option<&String>, validator: Validator) -> Block {
        let new_block_hash: String = Block::block_hash_from_params(index, &timestamp, bpm.clone(), prev_hash);
        Block{
            index: index,
            timestamp: timestamp.to_string(),
            bpm: bpm,
            hash: new_block_hash,
            prev_hash: prev_hash.cloned(),
            validator: validator
        }
    }
    pub fn generate(prev_block: Block, bpm: String, validator: Validator) -> Block{
        let timestamp: String = chrono_timestamp();
        let new_block_hash: String = Block::block_hash_from_params(prev_block.index + 1, &timestamp, bpm.clone(), Some(&prev_block.hash));
        Block { 
            index: prev_block.index + 1, 
            timestamp: timestamp, 
            bpm: bpm, 
            hash: new_block_hash, 
            prev_hash: Some(prev_block.hash), 
            validator: validator 
        }

    }
    pub fn validate(block: Block, prev_block: Block) -> bool {
        if prev_block.index + 1 != block.index {
            return false
        }
        if prev_block.hash != prev_block.prev_hash.unwrap() {
            return false
        }
        // only the genesis block is allowed to NOT have a prev_hash
        if block.prev_hash.is_none() && prev_block.index + 1 != 1{
            return  false
        }
        if Block::block_hash_from_instance(&block) != block.hash {
            return false
        }
        return true
    }
    pub fn to_string(&mut self) -> String{
        serde_json::to_string(self).expect("Failed to serialize Block to String!")
    }
    pub fn from_string(block: String) -> Block{
        serde_json::from_str(&block).expect("Failed to deserialized Block from String!")
    }
}



#[derive(Serialize, Deserialize, Clone)]
pub struct BlockChain{
    pub blocks: Vec<Block>
}

impl BlockChain{
    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
    pub fn to_string(&mut self) -> String{
        serde_json::to_string(self).expect("Failed to serialize Block to String!")
    }
    pub fn from_string(blockchain: String) -> BlockChain{
        serde_json::from_str(&blockchain).expect("Failed to deserialized Block from String!")
    }
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Validator{
    pub address: String,
    pub balance: u64
}

pub struct Vote{
    pub block: Block,
    pub stake: u64
}