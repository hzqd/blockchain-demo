use my_ext::kt_ext::KtStd;
use utils::coder::*;
use chrono::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct BlockHeader {
    pub time: i64,
    pub tx_hash: String,
    pub pre_hash: String,
}

#[derive(Debug)]
pub struct Block{
    pub header: BlockHeader,
    pub hash: String,
    pub data: String,
}

impl Block {
    fn set_hash(&mut self) {
        my_ser(&(self.header)).let_ref(|sh| self.hash = get_hash(&sh[..]))
    }

    pub fn new_block(data: String, pre_hash: String) -> Block {
        Block {
            header: BlockHeader {
                time: Utc::now().timestamp(),
                tx_hash: get_hash(&my_ser(&data)),
                pre_hash: pre_hash,
            },
            hash: String::new(),
            data: data,
        }.also_mut_ret_owned(|block| block.set_hash())
    }
}