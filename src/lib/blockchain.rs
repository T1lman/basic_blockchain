use crate::block::Block;

#[derive(Debug, Clone)]
pub struct Blockchain {
    req: u8,
    chain: Vec<Block>,
}

impl Blockchain {
    ///create instance
    pub fn new(req: u8) -> Self {
        Self {
            req,
            chain: Vec::new(),
        }
    }
    ///create first block
    pub fn genesis(&mut self, payload: Vec<u8>) {
        self.chain.push(Block::genesis(payload, self.req));
    }
    ///mine new block
    ///should only be called after a call to the genesis method
    pub fn mine_new_block(&mut self, payload: Vec<u8>) {
        let chainlen = self.chain.len();
        self.chain.push(Block::new(
            self.chain[chainlen - 1].clone(),
            payload,
            self.req,
        ));
    }
    /// get length of blockchain
    pub fn len(&self) -> usize {
        self.chain.len()
    }
    /// index blockchain and get block by value
    pub fn nth(&self, index: usize) -> Block {
        self.chain[index].clone()
    }
    /// index blockchain and get block by mut refenrence
    pub fn nth_mut_ref(&mut self, index: usize) -> &mut Block {
        &mut self.chain[index]
    }
    /// validate entire blockchain doesnt throw error
    /// returns false when chain is invalid
    pub fn validate(&mut self) -> bool {
        let mut prevhash = Block::hashblock(&self.chain[0]);
        for i in 1..self.len() {
            let currenthash = Block::hashblock(&self.chain[i]);
            if prevhash == self.chain[i].prevhash && currenthash == self.chain[i].currenthash {
                prevhash = Block::hashblock(&self.chain[i]);
            } else {
                return false;
            }
        }
        true
    }
}
