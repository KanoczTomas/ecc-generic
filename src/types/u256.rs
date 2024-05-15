use crate::types::U512;
use uint::construct_uint;


construct_uint!(
    pub struct U256(4);
);

impl std::convert::TryFrom<U512> for U256 {
    type Error = String;

    fn try_from(value: U512) -> Result<Self, Self::Error> {
        let U512(from) = value;
        let mut res = [0u64;4];
        res[0] = from[0];
        res[1] = from[1];
        res[2] = from[2];
        res[3] = from[3];
        //higher bits are present we return an error
        if from[4] | from[5] | from[6] | from [7] != 0 {
            return Err("Can not do a lossless conversion, as higher bits are non zero!".to_string())
        }
        Ok(U256(res))
    }
}