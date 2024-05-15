use crate::types::U256;
use uint::construct_uint;

construct_uint!(
    pub struct U512(8);
);

impl std::convert::From<U256> for U512 {
    fn from(value: U256) -> Self {
        let U256(from) = value;
        let mut res = [0u64;8];
        res[0] = from[0];
        res[1] = from[1];
        res[2] = from[2];
        res[3] = from[3];
        U512(res)
    }
}