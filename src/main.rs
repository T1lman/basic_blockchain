mod block;
mod blockchain;

fn main() {
    let payload = String::from("Hello genesis block").as_bytes().to_vec();
    let payload2 = String::from("Hello second block").as_bytes().to_vec();
    let mut blockchain = blockchain::Blockchain::new(5);
    blockchain.genesis(payload);
    blockchain.mine_new_block(payload2);
    println!("{:?}", blockchain.nth(0));
    println!("{:?}", blockchain.nth(1));
}
