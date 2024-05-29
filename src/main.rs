use std::vec;

use ecc_generic::types::{ECpoint, Scalar, Zp, EC, U256};
use rand::{thread_rng, Rng};
use ring::digest::{self, digest};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Secp256k1;
impl EC for Secp256k1 {
    const NAME: &'static str = "Secp256k1";
    //secp256k1
    const A: U256 = U256([0;4]);
    const B: U256 = U256([7, 0, 0, 0]);
    const N: U256 = U256([
        13822214165235122497,
        13451932020343611451,
        18446744073709551614,
        18446744073709551615,
        ]);
    const P: U256 = U256([
        18446744069414583343,
        18446744073709551615,
        18446744073709551615,
        18446744073709551615,
    ]);
}
#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Curve127p;
impl EC for Curve127p {
    const NAME: &'static str = "Curve127p";
    const A: U256 = U256([0;4]);
    const B: U256 = U256([7, 0, 0, 0]);
    const N: U256 = U256([127, 0, 0, 0]);
    const P: U256 = U256([127, 0, 0, 0]);
    
}
impl Curve127p {
    pub fn generator() -> ECpoint<Self> {
        let g = ECpoint::<Curve127p>::new(79, 64).unwrap();
        g
    }
    pub fn generate_point() -> ECpoint<Self> {
        loop {
            let x = thread_rng().gen_range(0..127);
            let x = Zp::<Self>::new(x);
            dbg!(x);
            let y = x.pow(3) + Zp::new(Curve127p::A)*x + Zp::new(Curve127p::B);
            let y = y.sqrt();
            dbg!(y);    
            if y.is_some() {
                println!("found point:");
                return ECpoint::<Self>::new(x, y.unwrap().0).unwrap();
            } else {
                println!("no square root for y, retrying...");
            }
        }
    }
    pub fn other_generator() -> ECpoint<Self> {
        let p = Curve127p::generate_point();
        p
    }
}

fn hash256(data: &[u8]) -> U256 {
    let d = digest(&digest::SHA512_256, data);
    U256::from(d.as_ref())
}

fn main() {
    type Zp = ecc_generic::types::Zp<Curve127p>;
    type Scalar = ecc_generic::types::Scalar<Curve127p>;
    // let scalar = |v| Scalar::new(v);
    let g = Curve127p::generator();
    let a = g * 5;
    let b = a / 5;
    dbg!(g, a, b);
    let inputs = [1, 2, 3, 5].map(|v| -v);//.map(scalar);
    let outputs = [8, 2];//.map(scalar);
    let fee = 1;
    dbg!(inputs.into_iter().sum::<i32>(), outputs.into_iter().sum::<i32>(), fee);
    let ins = inputs.iter().copied().map(|v| g * v).collect::<Vec<_>>();
    let mut outs = outputs.iter().copied().map(|v| g * v).collect::<Vec<_>>();
    outs.push(g * fee);
    dbg!(&ins, &outs);
    // dbg!([ins, outs].concat().into_iter().sum::<ECpoint<Curve127p>>());
    dbg!([&inputs[..], &outputs[..],&[fee]].concat().into_iter().sum::<i32>());
    let h = Curve127p::other_generator();
    dbg!(h);
    let random = |_| thread_rng().gen_range(0..127);
    let blinding_inputs = [0; 4].map(random).map(|v| -v);//.map(scalar);
    let blinding_outputs = [0; 2].map(random);//.map(scalar);
    let excess = [&blinding_inputs[..], &blinding_outputs[..]].concat().into_iter().sum::<i32>();
    dbg!(excess);
    dbg!(blinding_inputs, blinding_outputs);
    let bins = blinding_inputs.map(|v| h * v);
    dbg!(bins);
    let bouts = blinding_outputs.map(|v| h * v);
    dbg!(bouts);
    let public_excess = [&ins[..], &outs[..], &bins[..], &bouts[..]].concat().into_iter().sum::<ECpoint<Curve127p>>();
    dbg!(public_excess);
    dbg!(h * excess);
    let h = [
        public_excess.x().unwrap().0,
        public_excess.y().unwrap().0
        ].concat()
         .into_iter()
         .map(|v| v.to_ne_bytes())
         .flatten()
         .collect::<Vec<_>>();
    let h = hash256(&h[..]);
    println!("0x{:X}", h);
    dbg!(Scalar::new(h));
    dbg!(-223%127);
}