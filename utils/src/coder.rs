use bincode;
use my_ext::kt_ext::KtStd;
use serde::{Serialize, Deserialize};
use crypto::{digest::Digest, sha3::Sha3};

pub fn my_ser(value: &(impl Serialize + ?Sized)) -> Vec<u8> {
    bincode::serialize(value).unwrap()
}

pub fn my_des<'a, T>(bytes: &'a[u8]) -> T where T: Deserialize<'a> {
    bincode::deserialize(bytes).unwrap()
}

pub fn get_hash(v: &[u8]) ->String{
    Sha3::sha3_256().also_mut(|hasher| hasher.input(v)).result_str()
}

// for test:
#[cfg(test)]
mod tests {
    use crate::coder::*;
    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    struct Point {
        x: i32,
        y: i32,
    }
    #[test]
    fn coder_works() {
        let point = Point{ x:1, y:1 };
        let ser = my_ser(&point);
        let des: Point = my_des(&ser);
        assert_eq!(des, point);
    }
}