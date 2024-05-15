use std::marker::PhantomData;
use crate::types::{ECpoint, U256, GroupOrder};

pub trait EC: PartialEq + Default + Copy {
    const A: i64;
    const B: i64;
}

#[allow(non_snake_case)]
#[derive(Default, Debug)]
pub struct Curve<S, G: GroupOrder, E: EC> {
    pub name: String,
    pub a: E,
    pub b: E,
    pub G: ECpoint<G, E>,
    pub H: Option<ECpoint<G, E>>,
    pub N: U256,
    pub p: U256,
    pub h: u64,
    pub n: U256,
    _state: PhantomData<S>
}

#[derive(Default, Clone, Copy, PartialEq)]
pub struct FinalizedCurve;
#[derive(Default, Clone, Copy, PartialEq)]
pub struct UnfinalizedCurve;

#[allow(non_snake_case)]
impl<G: GroupOrder, E: EC> Curve<UnfinalizedCurve, G, E> {
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
    pub fn G<T: Into<ECpoint<G, E>>>(mut self, G: T) -> Self{
        self.G = G.into();
        self
    }
    pub fn H<T: Into<ECpoint<G, E>>>(mut self, H: T) -> Self{
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
    pub fn n<T: Into<U256>>(mut self, n: T) -> Self{
        self.n = n.into();
        self
    }
    pub fn finalize(self) -> Curve<FinalizedCurve, G, E>{
        Curve { name: self.name, 
            a: self.a, 
            b: self.b, 
            G: self.G, 
            H: self.H, 
            N: self.N, 
            p: G::P,
            h: self.h, 
            n: self.n, 
            _state: PhantomData}
    }
}
