use crate::block::Block;
use crate::blockchain::Blockchain;

/// test validation through modyfying the hash values of a block
#[test]
fn test_validation() {
    let payload = String::from("Hello genesis block").as_bytes().to_vec();
    let payload2 = String::from("Hello second block").as_bytes().to_vec();
    let payload3 = String::from("Hello third block").as_bytes().to_vec();
    let payload4 = String::from("Hello fourth block").as_bytes().to_vec();
    let mut blockchain = Blockchain::new(1);
    blockchain.genesis(payload);
    blockchain.mine_new_block(payload2);
    blockchain.mine_new_block(payload3);
    blockchain.mine_new_block(payload4);
    let blockref = blockchain.nth_mut_ref(1).currenthash = String::new();
    let blockref = blockchain.nth_mut_ref(2).prevhash = String::new();
    println!("{}", blockchain.validate());
}
