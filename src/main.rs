use utils::blockchain;

fn main() {
    let payload = String::from("Hello genesis block").as_bytes().to_vec();
    let payload2 = String::from("Hello second block").as_bytes().to_vec();
    let payload3 = String::from("Hello third block").as_bytes().to_vec();
    let payload4 = String::from("Hello fourth block").as_bytes().to_vec();
    let mut blockchain = blockchain::Blockchain::new(5);
    blockchain.genesis(payload);
    blockchain.mine_new_block(payload2);
    blockchain.mine_new_block(payload3);
    blockchain.mine_new_block(payload4);
    println!("{}", blockchain.validate())
}
