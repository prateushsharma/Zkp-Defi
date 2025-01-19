use sha2::{Sha256, Digest}; // Importing the Sha256 and Digest traits from sha2

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u32,            // The position of the block in the chain
    pub timestamp: u128,       // Timestamp of when the block was created
    pub data: String,          // Data contained in the block (like a transaction)
    pub previous_hash: String, // Hash of the previous block
    pub hash: String,          // Hash of this block
}

impl Block {
    // Function to calculate the hash of the block
    pub fn calculate_hash(&self) -> String {
        let block_data = format!(
            "{}{}{}{}", 
            self.index, self.timestamp, self.data, self.previous_hash
        );
        let mut hasher = Sha256::new();
        hasher.update(block_data);
        let result = hasher.finalize();
        format!("{:x}", result) // Format the result as a hexadecimal string
    }

    // Creates the genesis block (the first block in the chain)
    pub fn genesis_block() -> Self {
        Block {
            index: 0,
            timestamp: 0,
            data: String::from("Genesis Block"),
            previous_hash: String::from("0"),
            hash: String::from("genesis_hash"), // Placeholder hash for the genesis block
        }
    }

    // Constructor for creating a new block
    pub fn new(index: u32, data: String, previous_hash: &str) -> Self {
        let timestamp = 0; // Placeholder timestamp
        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash: previous_hash.to_string(),
            hash: String::new(),
        };
        block.hash = block.calculate_hash(); // Set the hash of the block
        block
    }
}

pub struct Blockchain {
    pub chain: Vec<Block>, // A vector to store the blocks in the chain
}

impl Blockchain {
    // Creates a new Blockchain with the genesis block
    pub fn new() -> Self {
        let mut chain = Vec::new();
        chain.push(Block::genesis_block()); // Add the genesis block to the chain
        Blockchain { chain }
    }

    // Adds a new block to the blockchain
    pub fn add_block(&mut self, data: String) {
        let last_block = self.chain.last().unwrap();
        let new_block = Block::new(last_block.index + 1, data, &last_block.hash);
        self.chain.push(new_block); // Add the new block to the chain
    }
}

fn main() {
    // Initialize a new blockchain
    let mut blockchain = Blockchain::new();

    // Add some blocks to the blockchain
    blockchain.add_block(String::from("First block after Genesis"));
    blockchain.add_block(String::from("Second block"));

    // Print the blockchain
    for block in blockchain.chain.iter() {
        println!("{:?}", block); // Print each block in the chain
    }
}
