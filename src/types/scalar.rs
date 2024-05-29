use std::{fmt::Debug, marker::PhantomData};
use rand::Rng;

use crate::types::{U256, U512, ECpoint, EC};

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Scalar<E: EC>(U256, PhantomData<E>);

impl<E: EC> Scalar<E> {
    pub fn new<T: Into<Scalar<E>>>(val: T) -> Self {
        let val: Scalar<E> = val.into();
        Self(val.0 % E::N, PhantomData)
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
    fn multiplicative_inverse(n: Scalar<E>) -> Scalar<E> {
        // from
        // https://github.com/paritytech/bigint/blob/master/src/uint.rs
        let p = E::N;
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

		Scalar::new(xy.0)
    }
    ///Generates a random scalar that fits the range <1, N)
    pub fn generate_secret() -> Scalar<E> {
        let mut secret = U256::zero();
        loop {
            rand::thread_rng().fill(&mut secret.0);
            if secret > 0.into() && secret < E::N  {
                return Self::new(secret)
            }
        }
    }
}

impl<E: EC> std::ops::Add for Scalar<E> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let (res, overflow) = self.0.overflowing_add(rhs.0);
        Scalar::new(if overflow || res >= E::N {
            res.overflowing_sub(E::N).0
        } else {
            res
        })
    }
}

impl<E: EC> std::ops::AddAssign for Scalar<E> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl<E: EC> std::ops::Neg for Scalar<E> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self.0.is_zero() {
            true => Scalar::zero(),
            false => Scalar::new(E::N - self.0)
        }
    }
}

impl<E: EC> std::ops::Sub for Scalar<E>{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl<E: EC> std::ops::SubAssign for Scalar<E>{
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl<E: EC> std::ops::Mul for Scalar<E> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let res = U512::from(self.unwrap()) * U512::from(rhs.unwrap());
        let res: U256 = (res % U512::from(E::N)).try_into().unwrap();//safe as we do modulo
        Scalar::new(res)
    }
}

impl<E: EC> std::ops::MulAssign for Scalar<E> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

impl<E: EC> std::ops::Div for Scalar<E> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * Scalar::multiplicative_inverse(rhs)
    }
}

impl<E: EC> std::ops::Mul<ECpoint<E>> for Scalar<E>{
    type Output = ECpoint<E>;

    fn mul(self, rhs: ECpoint<E>) -> Self::Output {
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

impl<E: EC> std::ops::DivAssign for Scalar<E> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}


macro_rules! impl_from_for_scalar_signed {
    ($($ti:ty,$tu:ty),*) => {
        $(
            impl<E: EC> std::convert::From<$ti> for Scalar<E>{
                fn from(value: $ti) -> Self {
                    match value >= 0 {
                        //$tu is the unsigned counterpart as U256 from is implemented on for them
                        true => Scalar(U256::from(value as $tu) % E::N, PhantomData),
                        false => -Scalar(U256::from(-value) % E::N, PhantomData)
                    }
                    
                }
            }
        )*
    };
}

impl_from_for_scalar_signed!(i8,u8,i16,u16,i32,u32,i64,u64,i128,u128);


macro_rules! impl_from_for_scalar_unsigned {
    ($($t:ty),*) => {
        $(
            impl<E: EC> std::convert::From<$t> for Scalar<E>{
                fn from(value: $t) -> Self {
                    Scalar(U256::from(value) % E::N, PhantomData)
                }
            }            
        )*
        
    };
}

impl_from_for_scalar_unsigned!(u8,u16,u32,u64,u128,U256);
impl<E: EC> std::iter::Sum for Scalar<E> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Scalar::zero(), |mut acc, elem| {
            acc += elem;
            acc
        })
    }
}