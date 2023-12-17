# Proof of Stake in Rust

This project is based on a [Go implementation](https://mycoralhealth.medium.com/code-your-own-proof-of-stake-blockchain-in-go-610cd99aa658) of Proof of Stake.

# Run with mock validators

```
cargo run
```
This will start the main loop with 10 validators that have stake balances from 0-100. Each validator will propose a block every 100 seconds that includes the weight (balance) of said validator. A block will be selected and added to the blocks.db database (that represents the chain's state). Proposed blocks are stored in candidates.db as a serialized `BlockChain`.

# Randomness

Blocks are chosen at random but validators with a larger `stake` have a higher chance of being selected. The `weights` are calculated based on this formula:

`validator_weight = (random_number * validator_stake) // total_stake`. The validator with the highest weight wins the round and gets to create the block.


# Espresso context

Espresso is based on a consensus protocol known as HotShot. An L1 smart contract acts as a StakeTable for consensus.

A sequencer will submit transactions to the consensus nodes and forward block commitments agreed upon by consensus to the L1 HotShot smart contract. The HotShot contract verifies Sequencer information and queries the StakeTable contract.

The Rollup nodes submit proofs to the L1 Rollup contract, that queries the HotShot contract for the state of the sequencing process.
