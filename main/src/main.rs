use my_ext::kt_ext::KtStd;
use core::blockchain::BlockChain;
use std::{thread, time::Duration};

fn mining(mut bc: BlockChain, message: &str) -> BlockChain {
    println!("start mining...");
    thread::sleep(Duration::from_secs(5));
    bc.add_block(message.to_string());
    println!("produce a block!\n");
    bc
}

fn main() {
    // (仅打印)创世区块:
    // BlockChain::new_blockchain().blocks.sout();

    // 创世:
    BlockChain::new_blockchain()
    
    // 第一次挖矿:
    .let_owned(|bc| mining(bc, "a -> b: 5 btc"))
    
    // 第二次挖矿:
    .let_owned(|bc| mining(bc, "c -> d: 7.5 btc"))
    
    // 打印区块
    .echo();
}
