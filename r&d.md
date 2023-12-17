# Espresso context

Espresso is based on a consensus protocol known as HotShot. An L1 smart contract acts as a StakeTable for consensus.

A sequencer will submit transactions to the consensus nodes and forward block commitments agreed upon by consensus to the L1 HotShot smart contract. The HotShot contract verifies Sequencer information and queries the StakeTable contract.

The Rollup nodes submit proofs to the L1 Rollup contract, that queries the HotShot contract for the state of the sequencing process.

# Open questions

How does the gossiping protocol ensure that that no block submissions are "lost" / that all nodes have received all proposals when it is time to run the lottery?

What if only x < n nodes have received a winning proposal?

# Todo

Implement a VRF & Node signatures.