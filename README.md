# Proof of Stake in Rust

This project is based on a [Go implementation](https://mycoralhealth.medium.com/code-your-own-proof-of-stake-blockchain-in-go-610cd99aa658) of Proof of Stake.

Since the original article serves as an introduction to Proof of Stake consensus, this Rust implementation is also not a production-ready network, but rather an artificial simulation of the decision-making process in a decentralized Proof of Stake network.

# Run with mock validators

```
cargo run
```
This will start the main loop with 10 validators that have stake balances from 0-100. Each validator will propose a block every 100 seconds that includes the weight (balance) of said validator. A block will be selected and added to the blocks.db database (that represents the chain's state). Proposed blocks are stored in candidates.db as a serialized `BlockChain`.

# Randomness

Blocks are chosen at random but validators with a larger `stake` have a higher chance of being selected. The `weights` are calculated based on this formula:

`validator_weight = (random_number * validator_stake) // total_stake`. The validator with the highest weight wins the round and gets to create the block.

![output](https://github.com/jonas089/pos-rust/blob/master/resources/output.png)

For each round, exactly one validator will be chosen to create the next block. Consensus is a vital component of every decentralized network and Proof of Stake is one of many consensus mechanisms that are being applied in blockchain networks.
