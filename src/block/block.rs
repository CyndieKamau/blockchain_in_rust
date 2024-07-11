use crate::header::*;
use crate::transaction::*;
use crate::block::merkle_tree::*;

use sha2::{Sha256, Digest};
use hex;

const MAXIMUM_BLOCK_CAPACITY: usize = 7;

#[derive(Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    capacity: usize,
}

impl Block {
    pub fn new_block(header: BlockHeader, transactions: Vec<Transaction>, capacity: usize) -> Block {
        Block {
            header,
            transactions: Vec::new(),
            capacity: MAXIMUM_BLOCK_CAPACITY,
        };
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), &'static str> {
        if self.transactions.len() < self.capacity {
            self.transactions.push(transaction);
            Ok(())
        } else {
            Err("Block is full")
        }
    }

    pub fn is_full(&self) -> bool {
        self.transactions.len() >= self.capacity
    }

    fn calculate_merkle_root(&self) -> String {
        let transaction_hashes: Vec<String> = self.transactions.iter().map(|tx| tx.calculate_hash()).collect();
        let merkle_tree = MerkleTree::new(transaction_hashes);
        merkle_tree.root().unwrap_or_else(|| String::new())
    }

    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();

        // Hash header fields
        hasher.update(self.header.index.to_string());
        hasher.update(self.header.timestamp.to_string());
        hasher.update(self.header.nonce.to_string());
        hasher.update(&self.header.prev_block_hash);
        hasher.update(self.header.difficulty.to_string());
        hasher.update(&self.header.merkle_root);

        let result = hasher.finalize();
        hex::encode(result)
    }

    pub fn finalize(&mut self) {
        let merkle_root = self.calculate_merkle_root();
        self.header.set_merkle_root(merkle_root);
        let block_hash = self.calculate_hash();
        self.header.set_current_block_hash(block_hash);
    }
}