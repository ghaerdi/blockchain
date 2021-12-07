use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time;

#[derive(Hash)]
struct HasheableDataBlock {
    data: String,
    timestamp: u64,
    nonce: u32,
}

impl HasheableDataBlock {
    fn new(data: String, timestamp: u64, nonce: u32) -> Self {
        HasheableDataBlock {
            data,
            timestamp,
            nonce,
        }
    }
}

#[derive(Debug)]
struct Block {
    data: String,
    timestamp: u64,
    hash: String,
    previous_hash: String,
    nonce: u32,
}

impl Block {
    pub fn new(data: &str, previous_hash: String) -> Block {
        let timestamp = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Block {
            data: data.to_string(),
            timestamp,
            hash: "0".to_string(),
            previous_hash,
            nonce: 0,
        }
    }

    pub fn calculate_hash<T: Hash>(data: T) -> String {
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        hasher.finish().to_string()
    }

    pub fn mine(data: &mut Self, difficulty: usize) {
        loop {
            let hash = Self::calculate_hash(HasheableDataBlock::new(
                data.data.clone(),
                data.timestamp,
                data.nonce,
            ));
            if hash.starts_with(&format!("{}", "1".repeat(difficulty))) {
                data.hash = hash;
                break;
            }
            data.nonce += 1;
        }
    }
}

struct IsValidResult {
    ok: bool,
    hash: String,
}

#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Blockchain {
        let mut chain = Vec::new();
        chain.push(Self::genesis(difficulty));

        Blockchain { chain, difficulty }
    }

    fn genesis(difficulty: usize) -> Block {
        let mut genesis = Block::new("Genesis Block", "0".to_string());
        Block::mine(&mut genesis, difficulty);
        return genesis;
    }

    pub fn add_block(&mut self, data: &str) {
        let last_block = self.chain.last().unwrap();
        let previous_hash = last_block.hash.clone();
        let mut new_block = Block::new(data, previous_hash);
        Block::mine(&mut new_block, self.difficulty);

        self.chain.push(new_block);
    }

    pub fn is_valid(&mut self) -> bool {
        let mut index = 1;

        while index < self.chain.len() {
            let current = &self.chain[index];
            let previous = &self.chain[index - 1];

            let check_previous = self.check_hash(&current, &previous);
            if !check_previous.ok {
                self.chain[index - 1].hash = check_previous.hash;
                return false;
            }

            let check_current = self.check_hash(&current, &current);
            if !check_current.ok {
                self.chain[index].hash = check_current.hash;
                return false;
            }

            index += 1;
        }
        return true;
    }

    fn check_hash(&self, current: &Block, with: &Block) -> IsValidResult {
        let hash = Block::calculate_hash(HasheableDataBlock::new(
            with.data.clone(),
            with.timestamp,
            with.nonce,
        ));

        if current.hash == with.hash {
            let check_hash = current.hash != hash;
            if check_hash {
                return IsValidResult { ok: false, hash };
            }
        } else {
            let check_hash = current.previous_hash != hash;
            if check_hash {
                return IsValidResult { ok: false, hash };
            }
        }

        return IsValidResult { ok: true, hash };
    }
}

fn main() {
    let mut blockchain = Blockchain::new(6);
    blockchain.add_block("First Block");
    blockchain.add_block("Second Block");

    // blockchain.chain[0].data = "Third Block".to_string();

    println!("{:#?}", blockchain);
    println!("is valid: {}", blockchain.is_valid());
}
