use sha2::{Sha256, Digest};
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64, // Add a nonce field for Proof of Work
}

impl Block {
    pub fn calculate_hash(&self) -> String {
        let block_data = format!(
            "{}{}{}{}{}", 
            self.index, self.timestamp, self.data, self.previous_hash, self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(block_data);
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    pub fn genesis_block() -> Self {
        Block {
            index: 0,
            timestamp: 0,
            data: String::from("Genesis Block"),
            previous_hash: String::from("0"),
            hash: String::from("genesis_hash"),
            nonce: 0, // Genesis block doesn't have a nonce
        }
    }

    pub fn new(index: u32, data: String, previous_hash: &str) -> Self {
        let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash: previous_hash.to_string(),
            hash: String::new(),
            nonce: 0,
        };
        block.hash = block.calculate_hash();
        block
    }

    // Proof of Work function (mining)
    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty); // Target hash starts with a certain number of zeroes
        while &self.hash[..difficulty] != target {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }
}

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize, // Difficulty of the Proof of Work
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let mut chain = Vec::new();
        chain.push(Block::genesis_block());
        Blockchain { chain, difficulty }
    }

    pub fn add_block(&mut self, data: String) {
        let last_block = self.chain.last().unwrap();
        let mut new_block = Block::new(last_block.index + 1, data, &last_block.hash);
        
        // Mine the block to find the valid nonce
        new_block.mine_block(self.difficulty);
        
        self.chain.push(new_block);
    }

    // Validate the blockchain by checking the hashes and the previous hashes
    pub fn validate(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            // Check if the current block's hash matches its calculated hash
            if current_block.hash != current_block.calculate_hash() {
                return false;
            }

            // Check if the previous block's hash matches the current block's previous_hash
            if current_block.previous_hash != previous_block.hash {
                return false;
            }

            // Ensure the block hash starts with the target number of zeroes (Proof of Work)
            if &current_block.hash[..self.difficulty] != "0".repeat(self.difficulty) {
                return false;
            }
        }
        true
    }
}

fn main() {
    // Create a new blockchain with difficulty level (the number of leading zeros in the hash)
    let mut blockchain = Blockchain::new(4);

    // Add some blocks to the blockchain
    blockchain.add_block(String::from("First block after Genesis"));
    blockchain.add_block(String::from("Second block"));

    // Print the blockchain
    for block in blockchain.chain.iter() {
        println!("{:?}", block);
    }

    // Validate the blockchain
    if blockchain.validate() {
        println!("Blockchain is valid!");
    } else {
        println!("Blockchain is invalid!");
    }
}
