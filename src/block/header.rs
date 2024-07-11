pub struct BlockHeader {
    pub index: u64,
    pub timestamp: u64,
    pub nonce: u64,
    pub current_block_hash: String,
    pub prev_block_hash: String,
    pub difficulty: u64,
    pub merkle_root: String,
}

impl BlockHeader {
    pub fn new_header(index: u64, timestamp: u64, nonce: u64, current_block_hash: String, prev_block_hash: String, difficulty: String, merkle_root: String) -> Self {
        
        
        BlockHeader {
            index,
            timestamp,
            nonce: 0,
            current_block_hash: String::new(),
            prev_block_hash,
            difficulty,
            merkle_root: String::new(),
        };

    }

    pub fn set_current_block_hash(&mut self, hash: String) {
        self.current_block_hash = hash;
    }

    pub fn set_merkle_root(&mut self, merkle_root: String) {
        self.merkle_root = merkle_root;
    }

    
        
}