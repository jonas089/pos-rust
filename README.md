# Proof of Stake in Rust

This project is based on a [Go implementation](https://mycoralhealth.medium.com/code-your-own-proof-of-stake-blockchain-in-go-610cd99aa658) of Proof of Stake.

# Run with mock validators

```
cargo run
```
This will start the main loop with 10 validators that have stake balances from 0-100. Each validator will propose a block every 100 seconds that includes the weight (balance) of said validator. A block will be selected and added to the blocks.db database (that represents the chain's state). Proposed blocks are stored in candidates.db as a serialized `BlockChain`.