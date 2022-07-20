use sha2::Digest;
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Block {
    pub timestamp: u128,
    pub payload: Vec<u8>,
    pub nonce: u64,
    pub currenthash: String,
    pub prevhash: String,
}

impl Block {
    /// create new blockchain neeeds a genesis block before this function call
    pub fn new(prevblock: Block, payload: Vec<u8>, req: u8) -> Self {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let mut block = Self {
            timestamp: time,
            payload,
            nonce: 0,
            currenthash: String::new(),
            prevhash: prevblock.currenthash,
        };
        let currhash = Self::hashblock(&block);
        block.currenthash = currhash;
        Self::mine(block, req)
    }
    /// create first ever block in a blockchain
    pub fn genesis(payload: Vec<u8>, req: u8) -> Self {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let mut block = Self {
            timestamp: time,
            payload,
            nonce: 0,
            currenthash: String::new(),
            prevhash: String::new(),
        };
        let currhash = Self::hashblock(&block);
        block.currenthash = currhash;
        Self::mine(block, req)
    }
    /// hashes block
    pub fn hashblock(block: &Self) -> String {
        let mut hasher = Sha256::new();
        let blockstring = format!(
            "{:#?}{:#?}{}{}",
            block.timestamp, block.payload, block.prevhash, block.nonce
        );
        hasher.update(&blockstring);
        format!("{:X}", hasher.finalize())
    }
    /// finds nonce for provided block and returns modified block
    fn mine(mut block: Self, req: u8) -> Block {
        fn check(hash: String, req: u8) -> bool {
            for i in 0..req {
                match hash.chars().nth(i as usize).unwrap() {
                    '0' => continue,
                    _ => return false,
                }
            }
            true
        }
        while !check(Self::hashblock(&block), req) {
            block.nonce += 1;
        }
        block.currenthash = Self::hashblock(&block);
        block
    }
}
