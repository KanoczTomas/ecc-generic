use std::io::{stdout, Write};
use rand::{thread_rng, Rng};

use ecc_generic::types::{ECpoint, GroupOrder, U256, EC, Zp};
#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct P;
impl GroupOrder for P {
    // const P: ecc_generic::types::U256 = U256::MAX;
    const P: U256 = U256([
        18446744069414583343,
        18446744073709551615,
        18446744073709551615,
        18446744073709551615,
    ]);
    // const P: U256 = U256([127, 0, 0, 0]);
}
#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Curve;
impl EC for Curve {
    const A: U256 = U256([0;4]);
    const B: U256 = U256([7, 0, 0, 0]);
    const N: U256 = U256([
        13822214165235122497,
        13451932020343611451,
        18446744073709551614,
        18446744073709551615,
    ]);

    fn order_of_cyclic_subgroup<G: GroupOrder, E: EC>(&self) -> U256 {
        todo!()
    }
    
    
    // const N: U256 = U256([0;4]);
}
fn main() {
    // let mut p;
    let mut x_p: Zp<P>;
    let mut y_p = Zp::zero();
    // let mut found = false;
    // for x in 0..u16::MAX {
    //     for y in 0..u16::MAX {
    //         println!("\rtrying x: {}, y: {}", x, y);
    //         let _ = stdout().flush();
    //         if ECpoint::<P, Curve>::new(x, y).is_some() {
    //             found = true;
    //             x_p = x;
    //             y_p = y;
    //         }
    //         if found {
    //             break
    //         }
    //     }
    //     if found {
    //         break
    //     }
    // }
    loop {
        // let _x_p = thread_rng().gen::<u32>();
        let mut _x_p = [0u64;4];
        thread_rng().fill(&mut _x_p);
        x_p = Zp::new(U256(_x_p));
        let _y_p = (x_p.pow(U256::from(3)) + Zp::new(Curve::A) * x_p + Zp::new(Curve::B)).sqrt();
        dbg!(x_p, y_p);
        if _y_p.is_none(){
            continue;
        }
        y_p = _y_p.unwrap();
        let p = ECpoint::<P, Curve>::new(x_p, y_p);
        println!("\rx: {}, y: {}", x_p.unwrap(), y_p.unwrap());
        let _ = stdout().flush();
        if p.is_some() {
            break;
        }
    }

    // let xp = "0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
    // let yp = "0x483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8";
    // let xp = U256::from_str_radix(&xp, 16).unwrap();
    // let yp = U256::from_str_radix(&yp, 16).unwrap();
    // dbg!(ECpoint::<P, Curve>::new(xp, yp));

    // let p = U256::from_str_radix("0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f", 16).unwrap();
    // dbg!(p.0);
    // dbg!(p);
    // let n = U256::from_str_radix("0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141", 16).unwrap();
    // dbg!(n.0);
    // dbg!(n);
    // let g_x = U256::from_str_radix("0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798", 16).unwrap();
    // dbg!(g_x.0);
    // dbg!(g_x);
}