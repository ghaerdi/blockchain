use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time;

#[derive(Hash)]
struct HasheableDataBlock {
    index: u64,
    data: String,
    timestamp: u64,
}

impl HasheableDataBlock {
    fn new(index: u64, data: String, timestamp: u64) -> Self {
        HasheableDataBlock {
            index,
            data,
            timestamp,
        }
    }
}

#[derive(Debug)]
struct Block {
    index: u64,
    data: String,
    timestamp: u64,
    hash: String,
    previous_hash: String,
}

impl Block {
    pub fn new(index: u64, data: &str, previous_hash: String) -> Block {
        let timestamp = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let hash =
            Self::calculate_hash(HasheableDataBlock::new(index, data.to_string(), timestamp));

        Block {
            index,
            data: data.to_string(),
            timestamp,
            hash,
            previous_hash: previous_hash.to_string(),
        }
    }

    pub fn calculate_hash<T: Hash>(data: T) -> String {
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        hasher.finish().to_string()
    }
}

#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>,
}

struct IsValidResult {
    ok: bool,
    hash: String,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let mut chain = Vec::new();
        chain.push(Self::genesis());

        Blockchain { chain }
    }
    fn genesis() -> Block {
        Block::new(0, "Genesis Block", "0".to_string())
    }
    pub fn add_block(&mut self, data: &str) {
        let last_block = self.chain.last().unwrap();
        let index = last_block.index + 1;
        let previous_hash = last_block.hash.clone();

        self.chain.push(Block::new(index, data, previous_hash));
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
                with.index,
                with.data.clone(),
                with.timestamp,
        ));

        if current.hash == with.hash {
            let check_hash = current.hash != hash;
            if check_hash {
                return IsValidResult{ ok: false, hash };
            }
        } else {
            let check_hash = current.previous_hash != hash;
            if check_hash {
                return IsValidResult{ ok: false, hash };
            }

        }

        return IsValidResult{ ok: true, hash };
    }
}

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block("First Block");
    blockchain.add_block("Second Block");

    blockchain.chain[0].data = "Third Block".to_string();

    println!("is valid: {}", blockchain.is_valid());

}
