// This project is part of my studies of the Espresso-sequencer which relies on HotShot consensus.


/*

    The main loop will reset the network and handle block creation.
    Verifiable randomness should be implemented to run the lottery / select a proposed block from the pool.
    Validator state / stake balances should be verifiable on-chain.
    Espresso uses a stake table contract to verify validators' stakes.
*/


mod helpers;
mod storage;
mod types;
use helpers::{chrono_timestamp, genesis_block, default_validator_set, initialize_blockstore_with_genesis, initialize_candidatestore, get_candidate_pool, get_block_with_height, purge_dbs};
use types::{Block, BlockChain, Validator, Vote};
use storage::{Storage, BlockStore, CandidateStore};
use dotenv::dotenv;
use std::{env, path::PathBuf};

use crate::helpers::hash_input;

fn main() {
    dotenv().ok();
    let block_db_path: String = env::var("DEFAULT_BLOCK_DB_PATH").expect("Failed to get DEFAULT_BLOCK_DB_PATH from env!");
    let candidate_db_path: String = env::var("DEFAULT_CANDIDATE_DB_PATH").expect("Failed to get DEFAULT_CANDIDATE_DB_PATH from env!");
    purge_dbs(PathBuf::from(&block_db_path), PathBuf::from(&candidate_db_path));
    let blocktime: u64 = env::var("DEFAULT_BLOCK_TIME").expect("Failed to get DEFAULT_BLOCK_TIME from env!").parse().expect("Failed to parse DEFAULT_BLOCK_TIME as u64!");
    let validators: Vec<Validator> = default_validator_set();
    let block_storage = Storage{
        path: PathBuf::from(&block_db_path)
    };
    let candidate_storage = Storage{
        path: PathBuf::from(&candidate_db_path)
    };
    initialize_blockstore_with_genesis(&block_storage);
    initialize_candidatestore(&candidate_storage);
    let mut height: u64 = 1;
    let mut round_participants: Vec<&Validator> = Vec::new();
    // the main loop that proposes blocks and decides on a new block every 30 seconds.
    loop{
        let prev_block: Block = get_block_with_height(&block_storage, &(height - 1));
        if &chrono_timestamp().parse::<u64>().unwrap() > &(prev_block.timestamp.parse::<u64>().unwrap() + blocktime){
            let pool: BlockChain = get_candidate_pool(&candidate_storage, &height);
            println!("{}", format!("It is time to run the lottery. {} Blocks have been proposed!", &pool.blocks.len()));

            // validators may propose blocks now -> time window for proposing blocks
            for validator in &validators{
                if round_participants.contains(&validator){
                    continue;
                }
                // generate a new block with a random bpm (bpm is a placeholder block payload)
                let block: Block = Block::generate(prev_block.clone(), hash_input(&chrono_timestamp()), validator.clone());
                // submit the block to the pool
                let _ = CandidateStore::insert(&candidate_storage, height, block);
                println!("{}", format!("Block has been added to pool by {} for round {}", &validator.address, &height));
                round_participants.push(&validator);
            };


            if pool.blocks.len() > 0{
                /*
                    * Select a random block from the pool and add it to blocks.db

                */
                let mut votes: Vec<Vote> = Vec::new();   
                let mut total_votes = 0;             
                for block in pool.blocks{
                    votes.push(Vote{
                        block: block.clone(),
                        stake: block.clone().validator.balance
                    });
                    total_votes += &block.validator.balance;
                };

                let lottery_winner = votes[0].block.clone();
                println!("{}", format!("Lottery winner: {}, timestamp: {}", &lottery_winner.validator.address, &lottery_winner.timestamp));
                let _ = BlockStore::insert(&block_storage, height, lottery_winner);

                // start next round
                height += 1;
                round_participants = Vec::new();
            }
        }
    }
}
