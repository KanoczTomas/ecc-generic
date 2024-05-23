use std::{fmt::Debug, marker::PhantomData};

use crate::types::{U256, U512, ECpoint, EC};

use super::GroupOrder;


pub trait CurveOrder: PartialEq + Default + Copy + Debug{
    const N: U256;    
}


#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Scalar<N: CurveOrder>(U256, PhantomData<N>);

impl<N: CurveOrder> Scalar<N> {
    pub fn new<T: Into<Scalar<N>>>(val: T) -> Self {
        let val: Scalar<N> = val.into();
        Self(val.0 % N::N, PhantomData)
    }
    pub fn zero() -> Self {
        Scalar::new(0)
    }
    pub fn one() -> Self {
        Scalar::new(1)
    }
    pub fn is_zero(&self) -> bool {
        *self == Scalar::zero()
    }
    pub fn is_one(&self) -> bool {
        *self == Scalar::one()
    }
    pub fn unwrap(&self) -> U256 {
        self.0
    }
}

impl<N: CurveOrder> std::ops::Add for Scalar<N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let (res, overflow) = self.0.overflowing_add(rhs.0);
        Scalar::new(if overflow || res >= N::N {
            res.overflowing_sub(N::N).0
        } else {
            res
        })
    }
}

impl<N: CurveOrder> std::ops::AddAssign for Scalar<N> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl<N: CurveOrder> std::ops::Neg for Scalar<N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self.0.is_zero() {
            true => Scalar::zero(),
            false => Scalar::new(N::N - self.0)
        }
    }
}

impl<N: CurveOrder> std::ops::Sub for Scalar<N>{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl<N: CurveOrder> std::ops::SubAssign for Scalar<N>{
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl<N: CurveOrder> std::ops::Mul for Scalar<N>{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let res = U512::from(self.unwrap()) * U512::from(rhs.unwrap());
        let res: U256 = (res % U512::from(N::N)).try_into().unwrap();//safe as we do modulo
        Scalar::new(res)
    }
}

impl<N: CurveOrder, E: EC, G: GroupOrder> std::ops::Mul<ECpoint<G, E>> for Scalar<N>{
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

impl<N: CurveOrder> std::ops::MulAssign for Scalar<N> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

macro_rules! impl_from_for_zp_signed {
    ($($ti:ty,$tu:ty),*) => {
        $(
            impl<N: CurveOrder> std::convert::From<$ti> for Scalar<N>{
                fn from(value: $ti) -> Self {
                    match value >= 0 {
                        //$tu is the unsigned counterpart as U256 from is implemented on for them
                        true => Scalar(U256::from(value as $tu), PhantomData),
                        false => -Scalar(U256::from(-value), PhantomData)
                    }
                    
                }
            }
        )*
    };
}

impl_from_for_zp_signed!(i8,u8,i16,u16,i32,u32,i64,u64,i128,u128);


macro_rules! impl_from_for_zp_unsigned {
    ($($t:ty),*) => {
        $(
            impl<N: CurveOrder> std::convert::From<$t> for Scalar<N>{
                fn from(value: $t) -> Self {
                    Scalar(U256::from(value), PhantomData)
                }
            }            
        )*
        
    };
}

impl_from_for_zp_unsigned!(u8,u16,u32,u64,u128,U256);