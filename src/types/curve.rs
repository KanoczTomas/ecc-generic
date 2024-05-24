use std::{fmt::Debug, marker::PhantomData};
use crate::{types::{ECpoint, U256}, utils::find_factors};


pub trait EC: PartialEq + Default + Copy{
    ///a constant in the elliptic curve equation
    const A: U256;
    ///b constant in the elliptic curve equation
    const B: U256;
    ///N constant is the order of the ellipcit curve, in other words
    ///how many points the generator generates
    const N: U256;
    ///P is the prime used in the Fp field the EC coordinates work in
    const P: U256;
    ///Finds random point P where nP = 0 and n != 1
    // fn find_generator<E: EC>(&mut self) -> ECpoint<G, E> {
    //     //pick random x coordinate
    //     let mut x = [0;4];
    //     self.fill(&mut x);
    //     let x = Zp::<G>::new(U256(x));
    //     ECpoint::Infinity
    // }

    ///Returns the number of curve points (naive default)
    ///implementation, goes through whole space and checks for 
    ///equation. For high P-s implement your own algo, e.g. 
    ///Schoof's algorithm
    fn n_curve_points<E: EC>(&self) -> U256 {
        let (mut count, mut x, mut y) = (U256::zero(), U256::zero(), U256::zero());
        while x != E::P  {
            while y != E::P  {
                if let Some(_) = ECpoint::<E>::new(x, y) {
                    count += U256::one();
                }
                y += U256::one();
            }
            x += U256::one();
            y = U256::zero();
        }
        count += U256::one(); //we add the point at infinity
        count
    }
    fn cofactor<E: EC>(&self) -> U256 {
        let a = find_factors(Self::n_curve_points::<E>(&self));
        dbg!(&a);
        a[0]
    }
    fn order_of_cyclic_subgroup<E: EC>(&self) -> U256; 
}

#[allow(non_snake_case)]
#[derive(Default, Debug)]
pub struct Curve<S, E: EC> {
    pub name: String,
    pub a: E,
    pub b: E,
    pub G: ECpoint<E>,
    pub H: Option<ECpoint<E>>,
    pub N: U256,
    pub p: U256,
    pub h: u64,
    // pub n: U256,
    _state: PhantomData<S>
}

#[derive(Default, Clone, Copy, PartialEq)]
pub struct FinalizedCurve;
#[derive(Default, Clone, Copy, PartialEq)]
pub struct UnfinalizedCurve;

#[allow(non_snake_case)]
impl<E: EC> Curve<UnfinalizedCurve, E> {
    pub fn new() -> Self {
        Curve::default()
    }
    // pub fn a<T: Into<i64>>(mut self, a: T) -> Self{
    //     self.a = a.into();
    //     self
    // }
    // pub fn b<T: Into<i64>>(mut self, b: T) -> Self{
    //     self.b = b.into();
    //     self
    // }
    pub fn G<T: Into<ECpoint<E>>>(mut self, G: T) -> Self{
        self.G = G.into();
        self
    }
    pub fn H<T: Into<ECpoint<E>>>(mut self, H: T) -> Self{
        let _ = self.H.insert(H.into());
        self
    }
    pub fn N<T: Into<U256>>(mut self, N: T) -> Self{
        self.N = N.into();
        self
    }
    pub fn h<T: Into<u64>>(mut self, h: T) -> Self{
        self.h = h.into();
        self
    }
    // pub fn n<T: Into<U256>>(mut self, n: T) -> Self{
    //     self.n = n.into();
    //     self
    // }
    pub fn finalize(self) -> Curve<FinalizedCurve, E>{
        Curve { name: self.name, 
            a: self.a, 
            b: self.b, 
            G: self.G, 
            H: self.H, 
            N: self.N, 
            p: E::P,
            h: self.h, 
            // n: N::N, 
            _state: PhantomData,
        }
    }
}
