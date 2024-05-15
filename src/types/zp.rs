use std::{fmt::Debug, marker::PhantomData};

use crate::types::{U256, U512};

use super::{ECpoint, EC};
/// Represents a skalar in the Zp field the ECC math is made in,
/// it is mod p (order of curve)
pub trait GroupOrder: PartialEq + Default + Copy + Debug{
    const P: U256;    
} 


#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Zp<G:GroupOrder>(U256, PhantomData<G>);

impl<G: GroupOrder> Zp<G> {
    pub const ZERO: Zp<G> = Zp(U256([0;4]), PhantomData);
    pub fn new<T: Into<Zp<G>>>(val: T) -> Self {
        let val: Zp<G> = val.into();
        Self(val.0 % G::P, PhantomData)
    }
    pub fn zero() -> Self {
        Zp::new(0)
    }
    pub fn one() -> Self {
        Zp::new(1)
    }
    pub fn is_zero(&self) -> bool {
        *self == Zp::zero()
    }
    pub fn is_one(&self) -> bool {
        *self == Zp::one()
    }
    pub fn unwrap(&self) -> U256 {
        self.0
    }
    ///We assume P is a prime!
    fn multiplicative_inverse(n: Zp<G>) -> Zp<G> {
        // from
        // https://github.com/paritytech/bigint/blob/master/src/uint.rs
        let p = G::P;
        let mut mn = (p, n.0);
		let mut xy = (U256::zero(), U256::one());

		while mn.1 != U256::zero() {
            let sb = U256::try_from(U512::from(mn.0 / mn.1) * U512::from(xy.1) % p).unwrap();
			if sb > xy.0 {
				xy = (xy.1, p - ((sb - xy.0) % p))
			} else {
				xy = (xy.1, xy.0 - sb)
			}
			mn = (mn.1, mn.0 % mn.1);
		}

		Zp::new(xy.0)
    }
    

}

impl<G: GroupOrder> std::ops::Add for Zp<G> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Zp::new(self.0 + rhs.0)
    }
}

impl<G: GroupOrder> std::ops::AddAssign for Zp<G> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl<G: GroupOrder> std::ops::Neg for Zp<G> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self.0.is_zero() {
            true => Zp::zero(),
            false => Zp::new(G::P - self.0)
        }
    }
}

impl<G: GroupOrder> std::ops::Sub for Zp<G>{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl<G: GroupOrder> std::ops::SubAssign for Zp<G>{
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl<G: GroupOrder> std::ops::Mul for Zp<G>{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Zp::new(self.0 % G::P * rhs.0 % G::P)
    }
}

impl<G: GroupOrder, E: EC> std::ops::Mul<ECpoint<G, E>> for Zp<G>{
    type Output = ECpoint<G, E>;

    fn mul(self, rhs: ECpoint<G, E>) -> Self::Output {
        let mut res = ECpoint::Infinity;
        let lhs = self.unwrap();
        let mut point = rhs;

        for b in 0..256 {
            if lhs.bit(b as usize) {
                res += point;
            }
            point += point; //doubleing
        }
        res
    }
}

impl<G: GroupOrder> std::ops::MulAssign for Zp<G> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

impl<G: GroupOrder> std::ops::Div for Zp<G> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * Zp::multiplicative_inverse(rhs)
    }
}

impl<G: GroupOrder> std::convert::From<i8> for Zp<G>{
    fn from(value: i8) -> Self {
        match value >= 0 {
            true => Zp(U256::from(value as u8), PhantomData),
            false => -Zp(U256::from(-value), PhantomData)
        }
        
    }
}


impl<G: GroupOrder> std::convert::From<i16> for Zp<G>{
    fn from(value: i16) -> Self {
        match value >= 0 {
            true => Zp(U256::from(value as u16), PhantomData),
            false => -Zp(U256::from(-value), PhantomData)
        }
        
    }
}


impl<G: GroupOrder> std::convert::From<i32> for Zp<G>{
    fn from(value: i32) -> Self {
        match value >= 0 {
            true => Zp(U256::from(value as u32), PhantomData),
            false => -Zp(U256::from(-value), PhantomData)
        }
        
    }
}


impl<G: GroupOrder> std::convert::From<i64> for Zp<G>{
    fn from(value: i64) -> Self {
        match value >= 0 {
            true => Zp(U256::from(value as u64), PhantomData),
            false => -Zp(U256::from(-value), PhantomData)
        }
        
    }
}

impl<G: GroupOrder> std::convert::From<i128> for Zp<G>{
    fn from(value: i128) -> Self {
        match value >= 0 {
            true => Zp(U256::from(value as u128), PhantomData),
            false => -Zp(U256::from(-value), PhantomData)
        }
        
    }
}

impl<G: GroupOrder> std::convert::From<u8> for Zp<G>{
    fn from(value: u8) -> Self {
        Zp(U256::from(value), PhantomData)
    }
}

impl<G: GroupOrder> std::convert::From<u16> for Zp<G>{
    fn from(value: u16) -> Self {
        Zp(U256::from(value), PhantomData)
    }
}

impl<G: GroupOrder> std::convert::From<u32> for Zp<G>{
    fn from(value: u32) -> Self {
        Zp(U256::from(value), PhantomData)
    }
}

impl<G: GroupOrder> std::convert::From<u64> for Zp<G>{
    fn from(value: u64) -> Self {
        Zp(U256::from(value), PhantomData)
    }
}

impl<G: GroupOrder> std::convert::From<u128> for Zp<G>{
    fn from(value: u128) -> Self {
        Zp(U256::from(value), PhantomData)
    }
}

impl<G: GroupOrder> std::convert::From<U256> for Zp<G>{
    fn from(value: U256) -> Self {
        Zp(value, PhantomData)
    }
}








