use std::marker::PhantomData;

use crate::types::{Zp, GroupOrder, EC, CurveOrder, Scalar};
#[derive(Default,Debug, PartialEq, Clone, Copy)]
///Represents a Point on curve that can be expressed with x,y coordinates
pub struct Point<G: GroupOrder, E: EC> {
    pub x: Zp<G>,
    pub y: Zp<G>,
    _curve: PhantomData<E>
}

#[derive(Default, Debug, PartialEq, Clone, Copy)]
///Represents the entire set of points, including inifnity
pub enum ECpoint<G: GroupOrder, E: EC> {
    #[default]
    Infinity,
    Point(Point<G, E>)
}

impl<G: GroupOrder, E: EC> ECpoint<G, E> {
    pub fn new<U: Into<Zp<G>>, T: Into<Zp<G>>>(x: U, y: T) -> Option<Self> {
        let (x, y) = (x.into(), y.into());
        let (a, b): (Zp<G>, Zp<G>) = (Zp::new(E::A), Zp::new(E::B));
        if y.pow(2) == x.pow(3) + a * x + b {
            Some(ECpoint::Point(Point{x, y, _curve: PhantomData}))
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
    pub fn get_point(&self) -> Option<&Point<G, E>> {
        match self {
            ECpoint::Infinity => None,
            ECpoint::Point(p) => Some(p)
        }
    }
    ///Warning, panics on infinity!
    pub fn x(&self) -> Zp<G> {
        match self {
            ECpoint::Infinity => panic!("Infinity has no x coordinate"),
            ECpoint::Point(p) => p.x
        }
    }
    ///Warning, panics on infinity!
    pub fn y(&self) -> Zp<G> {
        match self {
            ECpoint::Infinity => panic!("Infinity has no y coordinate"),
            ECpoint::Point(p) => p.y
        }
    }
}

impl<G: GroupOrder, E: EC> std::ops::Add for ECpoint<G, E> {
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

                ECpoint::Point(Point { x, y, _curve: PhantomData })
            }
        }
    }
}

impl<G: GroupOrder, E: EC> std::ops::AddAssign for ECpoint<G, E> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl<G: GroupOrder, E: EC> std::ops::Neg for ECpoint<G, E> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Point(Point { x: self.x(), y: -self.y(), _curve: PhantomData })
    }
}

impl<G: GroupOrder, E: EC> std::ops::Sub for ECpoint<G, E> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

///scalar multiplication
impl<G: GroupOrder, E: EC> std::ops::Mul<Scalar<E>> for ECpoint<G, E> {
    type Output = ECpoint<G, E>;
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

// macro_rules! impl_mut_for_ecpoint {
//     ($($t:ty),*) => {
//         $(
//             impl<G: GroupOrder, E: EC> std::ops::Mul<$t> for ECpoint<G, E> {
//                 type Output = ECpoint<G, E>;
            
//                 fn mul(self, rhs: $t) -> Self::Output {
//                     let rhs: Zp<G> = rhs.into();
//                     self * rhs
//                 }
//             }  
//         )*
//     };
// }

// for convenience with math, we do not own the built in types, so we can not write 2*P
// but using the below impls we can use a P*2
// impl_mut_for_ecpoint!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128);


// impl<G: GroupOrder, E: EC> std::ops::MulAssign<Zp<G>> for ECpoint<G, E> {
//     fn mul_assign(&mut self, rhs: Zp<G>) {
//         *self = *self * rhs
//     }
// }

// impl<G: GroupOrder, E: EC> std::ops::Div<Zp<G>> for ECpoint<G, E>{
//     type Output = ECpoint<G, E>;

//     fn div(self, rhs: Zp<G>) -> Self::Output {
//         (Zp::new(1)/rhs) * self
//     }
// }

// macro_rules! impl_div_for_ecpoint {
//     ($($t:ty),*) => {
//         $(
//             impl<G: GroupOrder, E: EC> std::ops::Div<$t> for ECpoint<G, E> {
//                 type Output = ECpoint<G, E>;
            
//                 fn div(self, rhs: $t) -> Self::Output {
//                     let rhs: Zp<G> = rhs.into();
//                     self / rhs
//                 }
//             }
//         )*
//     };
// }

// impl_div_for_ecpoint!(i8,u8,i16,u16,i32,u32,i64,u64,i128,u128,U256);

// impl<G: GroupOrder, E: EC> std::ops::DivAssign<Zp<G>> for ECpoint<G, E> {
//     fn div_assign(&mut self, rhs: Zp<G>) {
//         *self = *self / rhs
//     }
// }