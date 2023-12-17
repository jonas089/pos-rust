use sha2::{Sha256, Digest};
use hex;
use chrono::{DateTime, Local};
use crate::types::{Block, Validator, BlockChain};
use crate::storage::{Storage, BlockStore, CandidateStore};
use std::fs;
use std::path::{Path, PathBuf};
use rand::Rng;


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
            stake: 0
        }
    }
}

pub fn create_validator_set(n: u64, stakes: Vec<u64>) -> Vec<Validator>{
    let mut validators: Vec<Validator> = Vec::new();
    for i in 0..n{
        validators.push(Validator{
            address: format!("validator{}", i),
            stake: stakes[i as usize] 
        })
    }
    validators
}

pub fn default_validator_set() -> Vec<Validator>{
    let stakes: Vec<u64> = vec![10,20,30,40,50,60,70,80,90,100];
    create_validator_set(stakes.len() as u64, stakes)
}

pub fn initialize_blockstore_with_genesis(storage: &Storage){
    let _ = BlockStore::create(storage);
    let _ = BlockStore::insert(storage, 0, genesis_block());
}

pub fn initialize_candidatestore(storage: &Storage){
    let _ = CandidateStore::create(storage);
}

pub fn get_block_with_height(storage: &Storage, height: &u64) -> Block{
    let serialized_block = BlockStore::height(storage, height.clone()).unwrap().expect("Failed to get Block!");
    Block::from_string(serialized_block)
}

pub fn get_candidate_pool(storage: &Storage, height: &u64) -> BlockChain{
    let serialized_pool = CandidateStore::height(storage, height.clone()).unwrap().expect("Failed to get Pool!");
    BlockChain::from_string(serialized_pool)
}

pub fn purge_dbs(blockstorage: PathBuf, candidatestorage: PathBuf){
    if Path::new(&blockstorage).exists() {
        match fs::remove_file(blockstorage) {
            Ok(_) => println!("Block storage deleted successfully!"),
            Err(e) => eprintln!("Error deleting block storage, {:?}", e),
        }
    } else {
        println!("Warning: Block storage does not exist.");
    }

    if Path::new(&candidatestorage).exists() {
        match fs::remove_file(candidatestorage) {
            Ok(_) => println!("Candidate storage deleted successfully!"),
            Err(e) => eprintln!("Error deleting candidate storage, {:?}", e),
        }
    } else {
        println!("Warning: Candidate storage does not exist.");
    }
}

// not a VRF -> add VRF soon!
pub fn generate_random_number() -> u64{
    rand::thread_rng().gen()
}

// the weight of the validator + randomness
pub fn get_validator_weight(stake: u64, total_votes: u64) -> u64{
    generate_random_number() * (stake / total_votes)
}

pub fn generate_random_number_vrf(){
    todo!("Implement verifiable random function!");
}

