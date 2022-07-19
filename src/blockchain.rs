use crate::block::Block;

#[derive(Debug, Clone)]
pub struct Blockchain {
    req: u8,
    chain: Vec<Block>,
}

impl Blockchain {
    pub fn new(req: u8) -> Self {
        Self {
            req,
            chain: Vec::new(),
        }
    }
    pub fn genesis(&mut self, payload: Vec<u8>) {
        self.chain.push(Block::genesis(payload, self.req));
    }
    pub fn mine_new_block(&mut self, payload: Vec<u8>) {
        let chainlen = self.chain.len();
        self.chain.push(Block::new(
            self.chain[chainlen - 1].clone(),
            payload,
            self.req,
        ));
    }
    pub fn nth(&self, index: usize) -> Block {
        self.chain[index].clone()
    }
}
