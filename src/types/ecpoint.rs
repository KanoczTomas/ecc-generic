use crate::types::{Zp, EC, Scalar, U256};
#[derive(Default, PartialEq, Clone, Copy)]
///Represents a Point on curve that can be expressed with x,y coordinates
pub struct Point<E: EC> {
    x: Zp<E>,
    y: Zp<E>,
}

impl<E: EC> std::fmt::Debug for Point<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.debug_struct("Point").field("x", &self.x).field("y", &self.y).finish()
        write!(f, "Point on curve {} => ({}, {}) [mod {}]", E::NAME, self.x, self.y, E::P)?;
        Ok(())
    }
}

#[derive(Default, PartialEq, Clone, Copy)]
///Represents the entire set of points lying on EC, including inifnity
pub enum ECpoint<E: EC> {
    #[default]
    Infinity,
    Point(Point<E>)
}

impl<E: EC> std::fmt::Debug for ECpoint<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Infinity => write!(f, "Point at Infinity"),
            // Self::Point(arg0) => f.debug_tuple("Point").field(arg0).finish(),
            Self::Point(p) => write!(f, "{:?}", p)
        }
    }
}

impl<E: EC> ECpoint<E> {
    pub fn new<U: Into<Zp<E>>, T: Into<Zp<E>>>(x: U, y: T) -> Option<Self> {
        let (x, y) = (x.into(), y.into());
        let (a, b): (Zp<E>, Zp<E>) = (Zp::new(E::A), Zp::new(E::B));
        if y.pow(2) == x.pow(3) + a * x + b {
            Some(ECpoint::Point(Point{x, y}))
        } else {
            None
        }
    }
    pub fn is_infinity(&self) -> bool {
        match self {
            Self::Infinity => true,
            Self::Point(_) => false
        }
    }
    pub fn get_point(&self) -> Option<&Point<E>> {
        match self {
            ECpoint::Infinity => None,
            ECpoint::Point(p) => Some(p)
        }
    }
    ///Warning, panics on infinity!
    pub fn x(&self) -> Zp<E> {
        match self {
            ECpoint::Infinity => panic!("Infinity has no x coordinate"),
            ECpoint::Point(p) => p.x
        }
    }
    ///Warning, panics on infinity!
    pub fn y(&self) -> Zp<E> {
        match self {
            ECpoint::Infinity => panic!("Infinity has no y coordinate"),
            ECpoint::Point(p) => p.y
        }
    }
}

impl<E: EC> std::ops::Add for ECpoint<E> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (&ECpoint::Infinity, &ECpoint::Infinity) => ECpoint::Infinity, // 0 + 0 = 0
            (&ECpoint::Infinity, _) => rhs, //0 + rhs = rhs
            (_, &ECpoint::Infinity) => self, //self + 0 = 0
            (&a, &b) if a == -b => ECpoint::Infinity, //a + (-a) = 0
            _ => {
                let q = self;
                let p = rhs;

                let lambda = if p == q {
                    // point doubling
                    (p.x() * p.x() * Zp::new(3) + Zp::new(E::A)) / (p.y() * Zp::new(2))
                } else {
                    (q.y() - p.y()) / (q.x() - p.x())
                };

                let x = lambda * lambda - p.x() - q.x();
                let y = lambda * (p.x() - x) - p.y();

                ECpoint::Point(Point { x, y})
            }
        }
    }
}

impl<E: EC> std::ops::AddAssign for ECpoint<E> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl<E: EC> std::ops::Neg for ECpoint<E> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Point(Point { x: self.x(), y: -self.y() })
    }
}

impl<E: EC> std::ops::Sub for ECpoint<E> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

///scalar multiplication
impl<E: EC> std::ops::Mul<Scalar<E>> for ECpoint<E> {
    type Output = ECpoint<E>;
    ///the scalar is modulo N, the order of the elliptic curve!
    fn mul(self, rhs: Scalar<E>) -> Self::Output {
        let mut res = ECpoint::Infinity;
        let rhs = rhs.unwrap();
        let mut point = self;

        for b in 0..256 {
            if rhs.bit(b as usize) {
                res += point;
            }
            point += point; //doubleing
        }
        res
    }
}
impl<E: EC> std::ops::MulAssign<Scalar<E>> for ECpoint<E> {
    fn mul_assign(&mut self, rhs: Scalar<E>) {
        *self = *self * rhs
    }
}



macro_rules! impl_mut_for_ecpoint {
    ($($t:ty),*) => {
        $(
            impl<E: EC> std::ops::Mul<$t> for ECpoint<E> {
                type Output = ECpoint<E>;
            
                fn mul(self, rhs: $t) -> Self::Output {
                    let rhs: Scalar<E> = rhs.into();
                    self * rhs
                }
            }  
        )*
    };
}

// for convenience with math, we do not own the built in types, so we can not write 2*P
// but using the below impls we can use a P*2
impl_mut_for_ecpoint!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128);


impl<E: EC> std::ops::Div<Scalar<E>> for ECpoint<E>{
    type Output = ECpoint<E>;

    fn div(self, rhs: Scalar<E>) -> Self::Output {
        (Scalar::new(1)/rhs) * self
    }
}

macro_rules! impl_div_for_ecpoint {
    ($($t:ty),*) => {
        $(
            impl<E: EC> std::ops::Div<$t> for ECpoint<E> {
                type Output = ECpoint<E>;
            
                fn div(self, rhs: $t) -> Self::Output {
                    let rhs: Scalar<E> = rhs.into();
                    self / rhs
                }
            }
        )*
    };
}

impl_div_for_ecpoint!(i8,u8,i16,u16,i32,u32,i64,u64,i128,u128,U256);

impl<E: EC> std::ops::DivAssign<Scalar<E>> for ECpoint<E> {
    fn div_assign(&mut self, rhs: Scalar<E>) {
        *self = *self / rhs
    }
}

impl<E: EC> std::iter::Sum for ECpoint<E> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(ECpoint::Infinity, |mut acc, point|{
            acc += point;
            acc
        })
    }
}