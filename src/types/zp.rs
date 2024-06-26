use std::marker::PhantomData;

use crate::types::{U256, U512, EC};

#[derive(Clone, Copy, PartialEq, Default)]
pub struct Zp<E: EC>(U256, PhantomData<E>);

impl<E: EC> std::fmt::Display for Zp<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",self.0)?;
        Ok(())
    }
}

impl<E: EC> std::fmt::Debug for Zp<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.debug_tuple("Zp").field(&self.0).field(&self.1).finish()
        write!(f, "{} [mod {}]", self.0, E::P)?;
        Ok(())
    }
}

impl<E: EC> Zp<E> {
    pub const ZERO: Zp<E> = Zp(U256([0;4]), PhantomData);
    pub fn new<T: Into<Zp<E>>>(val: T) -> Self {
        let val: Zp<E> = val.into();
        Self(val.0 % E::P, PhantomData)
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
    ///Find the only number m, such that n * m = 1 mod P
    ///We assume P is a prime!
    fn multiplicative_inverse(n: Zp<E>) -> Zp<E> {
        // from
        // https://github.com/paritytech/bigint/blob/master/src/uint.rs
        let p = E::P;
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
    ///Raises self to the power of exp using square and multiply algorithm.
    pub fn pow<T: Into<U256>>(self, exp: T) -> Zp<E> {
        let mut base = U512::from(self.unwrap());
        let mut exp = exp.into();
        let mut res = U512::one();
        let p = U512::from(E::P);
        while exp != 0.into() {
            if exp & 1.into() == 1.into() {
                res = (res * base) % p; //multiply
            }
            base = (base * base) % p; //square
            exp >>= 1; //devide by half
        }
        let res: U256 = res.try_into().unwrap();
        Zp::new(res)
    }
    ///Decides whether a number is a quadratic residue. If it has a square root
    ///it is a quadratic residue mod p. We use Euler's criterion to do so.
    pub fn is_quadratic_residue(self) -> bool {
        //mod 2, trivial results
        if E::P == 2.into() {
            return true
        }
        //if self % p == 0
        //As Zp is already mod p, we just have to check if it is 0
        match self.is_zero() {
            true => true,
            false => self.pow((E::P- 1) / 2) == 1.into()
        }

    }
    ///Find number n such that n * n = self. In other words finds the square root 
    ///modulo prime of self. This algorithm returns (n, -n) mod p
    pub fn sqrt(self) -> Option<(Self, Self)> {
        //rewrite of
        //https://github.com/jacksoninfosec/tonelli-shanks/blob/main/tonelli-shanks.py
        //if n % p == 0, 0 is a trivial solution
        if self.is_zero() {
            return Some((self, self))
        }
        if self.is_quadratic_residue() == false {
            return None
        }
        //If p=3(mod 4) and we know n is a quadratic residue then 
        //we can solve x^2=n(mod p) directly
        if E::P % U256::from(4) == 3.into() {
            let res = self.pow((E::P+1)/4);
            return Some((res, -res));
        }
        //So now p=1(mod 4), (although this is not needed in the algorithm).
        //Write p - 1 = (2^S)(Q) where Q is odd
        #[allow(non_snake_case)]
        let mut Q = E::P - 1;
        #[allow(non_snake_case)]
        let mut S = U256::zero();
        while Q % U256::from(2) == 0.into() {
            S += U256::from(1);
            Q /= U256::from(2);
        }
        //Find a quadratic non-residue of p by brute force search
        let mut z = Zp::new(2);
        while z.is_quadratic_residue() {
            z += 1.into()
        }

        //Initialize variables
        #[allow(non_snake_case)]
	    let mut M = S;
	    let mut c = z.pow(Q);
	    let mut t = self.pow(Q);
        #[allow(non_snake_case)]
	    let mut R = self.pow((Q+1)/2);
        while t != 1.into() {
            //Calculate i
            let mut i = U256::zero();
            let mut temp = t; 
            while temp != 1.into(){
                i += U256::one();
                temp *= temp;
            }
            
            //Calculate b, M, c, t, R
            let pow2 = Zp::<E>::new(2).pow(M - i - 1);
            let b = c.pow(pow2.unwrap());
            M = i;
            c = b * b;
            t = t * b * b;
            R = R * b;
            
        }
        return Some((R, -R))
    }
    

}

impl<E: EC> std::ops::Add for Zp<E> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let (res, overflow) = self.0.overflowing_add(rhs.0);
        Zp::new(if overflow || res >= E::P {
            res.overflowing_sub(E::P).0
        } else {
            res
        })
    }
}

impl<E: EC> std::ops::AddAssign for Zp<E> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl<E: EC> std::ops::Neg for Zp<E> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self.0.is_zero() {
            true => Zp::zero(),
            false => Zp::new(E::P - self.0)
        }
    }
}

impl<E: EC> std::ops::Sub for Zp<E>{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl<E: EC> std::ops::SubAssign for Zp<E>{
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl<E: EC> std::ops::Mul for Zp<E>{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let res = U512::from(self.unwrap()) * U512::from(rhs.unwrap());
        let res: U256 = (res % U512::from(E::P)).try_into().unwrap();//safe as we do modulo
        Zp::new(res)
    }
}

impl<E: EC> std::ops::MulAssign for Zp<E> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

impl<E: EC> std::ops::Div for Zp<E> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * Zp::multiplicative_inverse(rhs)
    }
}

macro_rules! impl_from_for_zp_signed {
    ($($ti:ty,$tu:ty),*) => {
        $(
            impl<E: EC> std::convert::From<$ti> for Zp<E>{
                fn from(value: $ti) -> Self {
                    match value >= 0 {
                        //$tu is the unsigned counterpart as U256 from is implemented on for them
                        true => Zp(U256::from(value as $tu) % E::P, PhantomData),
                        false => -Zp(U256::from(-value) % E::P, PhantomData)
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
            impl<E: EC> std::convert::From<$t> for Zp<E>{
                fn from(value: $t) -> Self {
                    Zp(U256::from(value) % E::P, PhantomData)
                }
            }            
        )*
        
    };
}

impl_from_for_zp_unsigned!(u8,u16,u32,u64,u128,U256);