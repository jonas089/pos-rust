use rocket::time::format_description::modifier::UnixTimestamp;
use rusqlite;
use rusqlite::{Connection, Result};
use std::path::PathBuf;
use dotenv::dotenv;
use std::env;
use crate::helpers::hash_input;
use crate::types::{Block, BlockChain};

pub trait BlockStore{
    fn create(&self) -> Result<()>;
    fn insert(&self, height: u64, block: Block) -> Result<()>;
    fn height(&self, height: u64)-> Result<Option<String>>;
}

pub trait CandidateStore{
    fn create(&self) -> Result<()>;
    fn insert(&self, height: u64, block: Block) -> Result<()>;
    fn height(&self, height: u64) -> Result<Option<String>>;
}

pub struct Storage{
    pub path: PathBuf
}

impl BlockStore for Storage{
    fn create(&self) -> Result<()> {
        let conn: Connection = Connection::open(&self.path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS data (
                    id INTEGER PRIMARY KEY,
                    height INTEGER,
                    block TEXT NOT NULL
                )",
            [],
        )?;
        Ok(())
    }
    fn insert(&self, height: u64, mut block: Block) -> Result<()> {
        let conn: Connection = Connection::open(&self.path)?;
        conn.execute(
            "INSERT INTO data (height, block) VALUES (?1, ?2)",
            &[&height.to_string(), &block.to_string()],
        )?;
        Ok(())
    }
    fn height(&self, height: u64) -> Result<Option<String>> {
        let conn: Connection = Connection::open(&self.path)?;
        let mut stmt: rusqlite::Statement<'_> = conn
            .prepare("SELECT height, block FROM data WHERE height = ?1 LIMIT 1")?;
        // Use query_row and check for QueryReturnedNoRows error
        match stmt.query_row(&[&height], |row| {
            let block: String = row.get(1)?;
            Ok(block)
        }) {
            Ok(b) => Ok(Some(b)),
            Err(err) => Ok(None),
        }
    }
}

impl CandidateStore for Storage{
    fn create(&self) -> Result<()> {
        let conn: Connection = Connection::open(&self.path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS data (
                    id INTEGER PRIMARY KEY,
                    height INTEGER,
                    blocks TEXT NOT NULL
                )",
            [],
        )?;
        Ok(())
    }
    fn insert(&self, height: u64, block: Block) -> Result<()>{
        let candidates_serialized: Option<String> = CandidateStore::height(self, height).expect("Failed to get pool from CandidateStore!");
        let mut is_first_entry: bool = bool::default();
        let mut candidates: BlockChain = match candidates_serialized{
            Some(candidates) => {
                BlockChain::from_string(candidates)
            },
            None => {
                is_first_entry = true;
                BlockChain{
                    blocks: Vec::new()
                }
            }
        };
        println!("Current candidates: {:?}", &candidates.blocks.len());
        candidates.add_block(block);
        let conn: Connection = Connection::open(&self.path)?;
        if is_first_entry{
            conn.execute(
                "INSERT INTO data (height, blocks) VALUES (?1, ?2)",
                &[&height.to_string(), &candidates.to_string()],
            )?;
        }
        else {
            conn.execute(
                "UPDATE data SET blocks = ?2 WHERE height = ?1",
                &[&height.to_string(), &candidates.to_string()],
            )?;
        }
        Ok(())
    }
    fn height(&self, height: u64) -> Result<Option<String>> {
        let conn: Connection = Connection::open(&self.path)?;
        let mut stmt: rusqlite::Statement<'_> = conn
            .prepare("SELECT height, blocks FROM data WHERE height = ?1 LIMIT 1")?;
        // Use query_row and check for QueryReturnedNoRows error
        match stmt.query_row(&[&height], |row| {
            let blocks: String = row.get(1)?;
            Ok(blocks)
        }) {
            Ok(b) => Ok(Some(b)),
            Err(err) => Ok(None),
        }
    }
}

#[test]
fn test_block_store(){
    use crate::helpers::genesis_block;
    dotenv().ok();
    let block_db_path = env::var("DEFAULT_BLOCK_DB_PATH").expect("Failed to get Block db path from env!");
    let storage = Storage{
        path: PathBuf::from(block_db_path.clone())
    };
    // will fail if db already exists -> not unwrapped here.
    let msg = format!("Failed to create db file at: {:?}!", &block_db_path);
    let _ = BlockStore::create(&storage).expect(&msg);
    let genesis_block = genesis_block();
    let _ = BlockStore::insert(&storage, 0, genesis_block);
    let block = BlockStore::height(&storage, 0).unwrap();
    println!("Block: {:?}", &block);
}

#[test]
fn test_candidate_store(){
    use crate::helpers::{chrono_timestamp, genesis_block, create_validator_set};
    use crate::types::Validator;
    dotenv().ok();
    let candidate_db_path = env::var("DEFAULT_CANDIDATE_DB_PATH").expect("Failed to get Candidate db path from env!");
    let storage = Storage{
        path: PathBuf::from(candidate_db_path.clone())
    };
    let msg = format!("Failed to create db file at: {:?}!", &candidate_db_path);
    let _ = CandidateStore::create(&storage).expect(&msg);
    let balances: Vec<u64> = vec![10,20,30,40,50,60,70,80,90,100];
    let validators: Vec<Validator> = create_validator_set(balances.len() as u64, balances);
    let genesis_block: Block = genesis_block();
    // each validator creates a block proposal
    for validator in validators{
        // propose blocks for height 1
        let _ = CandidateStore::insert(&storage, 1, Block::generate(genesis_block.clone(), hash_input(&chrono_timestamp()), validator));
        // sleep for .1 seconds
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    // obtain the current pool and print it
    let pool = CandidateStore::height(&storage, 1).unwrap();
    println!("Current pool: {:?}", &pool);
}