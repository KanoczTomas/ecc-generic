use std::{iter::once, vec};
use ecc_generic::types::{ECpoint, Zp, EC, U256};
use rand::{thread_rng, Rng};
use ring::digest::{self, digest};
use colored::Colorize;

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
impl std::fmt::Display for Secp256k1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Secp256k1 {
    pub fn generator() -> ECpoint<Self> {
        if Self::P == 10_729.into() {
            return ECpoint::new(6898, 2464).unwrap()
        }
        ECpoint::new(
            U256::from_dec_str("68987929337165961625603604241159030720925183355951772200563078805252891814032").unwrap(),
            U256::from_dec_str("74922205077911033266530596195997136488708902915259859854522397471577419437459").unwrap()
        ).unwrap()
    }
}

#[derive(Default, Clone, Copy, PartialEq)]
struct Curve10729p;
impl EC for Curve10729p {
    const NAME: &'static str = "Curve10729p";
    const A: U256 = U256([0;4]);
    const B: U256 = U256([7, 0, 0, 0]);
    // // const N: U256 = U256([127, 0, 0, 0]);
    // // const N: U256 = U256([313, 0, 0, 0]);
    const N: U256 = U256([10687, 0, 0, 0]);
    // // const P: U256 = U256([127, 0, 0, 0]);
    // // const P: U256 = U256([349, 0, 0, 0]);
    const P: U256 = U256([10729, 0, 0, 0]);
}

impl std::fmt::Debug for Curve10729p {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.debug_struct("Curve10729p").finish()
        writeln!(f, "Name: {}",Self::NAME)?;
        writeln!(f, "A: {}",Self::A)?;
        writeln!(f, "B: {}",Self::B)?;
        writeln!(f, "N: {}",Self::N)?;
        writeln!(f, "P: {}",Self::P)?;
        Ok(())
    }
}


impl Curve10729p {
    pub fn generator() -> ECpoint<Self> {
        if Self::P == 10_729.into() {
            return ECpoint::new(6898, 2464).unwrap()
        }
        ECpoint::new(
            U256::from_dec_str("68987929337165961625603604241159030720925183355951772200563078805252891814032").unwrap(),
            U256::from_dec_str("74922205077911033266530596195997136488708902915259859854522397471577419437459").unwrap()
        ).unwrap()
    }
    pub fn generate_point() -> ECpoint<Self> {
        println!("generate_point invoked ...");
        loop {
            // let x = thread_rng().gen_range(0..Self::P.to_string().parse().unwrap());
            let mut x = [0u64;4];
            thread_rng().fill(&mut x);
            let x = Zp::<Self>::new(U256(x));
            dbg!(x);
            let y = x.pow(3) + Zp::new(Self::A)*x + Zp::new(Self::B);
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
        #![allow(non_snake_case)]
        let zp_to_u8_iter = |zp: Zp<Self>| zp
            .unwrap()
            .0
            .into_iter()
            .map(|v| v.to_be_bytes())
            .flatten();
        let G = Self::generator();
        // let g_encoding = &[
        //     vec![0x04],
        //     zp_to_u8_iter(G.x()),
        //     zp_to_u8_iter(G.y())
        //     ].concat()[..]; //uncompressed pub key format 0x04+[BE X]+[BE Y] coordinate
        let g_encoding = vec![0x04u8]
            .into_iter()
            .chain(zp_to_u8_iter(G.x()))
            .chain(zp_to_u8_iter(G.y()))
            .collect::<Vec<_>>();
        let mut counter = 0u8;
        let mut hx = Zp::<Self>::zero();
        let mut hy = None;
        while let None = hy {
            dbg!(counter);
            // let preimage = &[g_encoding, &[counter][..]].concat()[..];
            let preimage = g_encoding.clone().into_iter().chain(once(counter)).collect::<Vec<_>>();
            let hash = hash256(preimage.as_slice());
            println!("hash of generator X coordinate: 0x{:X}", hash);
            hx = Zp::new(hash);
            dbg!(&hx);
            hy = (hx.pow(3) + Zp::new(Self::A) + Zp::new(Self::B)).sqrt();
            dbg!(&hy);
            counter += 1;
        }
        ECpoint::new(hx, hy.unwrap().0).unwrap()
    }
}

fn hash256(data: &[u8]) -> U256 {
    let d = digest(&digest::SHA256, data);
    U256::from(d.as_ref())
}

fn random<T: Into<U256>>(_a: T) -> U256 {
    let mut n = [0u64;4];
    thread_rng().fill(&mut n);
    U256(n)
}

fn silent_payment_demo() {
    #![allow(non_snake_case)]
    type Scalar = ecc_generic::types::Scalar<Secp256k1>;
    println!("{}", "-".repeat(30));
    println!("{}", "silent payments demo:".green());
    println!("{}", "-".repeat(30));
    println!("{}", "Elliptic curve params:".green());
    print!("{}", format!("{:?}", Curve10729p).red());
    println!("{}", "Alice sends payment to Bob ...".green());
    let G = Secp256k1::generator();
    let bob_priv = Some(0).map(random).map(Scalar::new).unwrap();
    let bob_pub_key = G * bob_priv;
    let alice_priv = Some(0).map(random).map(Scalar::new).unwrap();
    let alice_pub_key = G * alice_priv;
    println!("{}{}", "b = ".red(), format!("{:?}", bob_priv).red());
    println!("{}{}", "B = ".red(), format!("{:?}", bob_pub_key).red());
    println!("{}{}", "a = ".red(), format!("{:?}", alice_priv).red());
    println!("{}{}", "A = ".red(), format!("{:?}", alice_pub_key).red());
    
    println!("{}{}", "Bob sends his sp address, where he has his pub key ".green(), format!("B = {:?}", bob_pub_key).red());
    println!("{}{}{}{}{}", "Alice computes ".green(), "P = B + H(a*B)*G".red(), ". As ".green(), "a*B == b*A".red(), ",".green());
    println!("{}{}{}{}{}", "Bob can now precompute ".green(), "p".red(), ", the private key for ".green(), "P".red(), ", as he has all the info needed!".green());
    println!("{}{}{}{}", "Aserting ".green(), "a*B == b*A".red(), ":\n".green(), format!("=>\t{:?} == {:?}", bob_pub_key * alice_priv, alice_pub_key * bob_priv).red());
    let hAb = hash256(&(alice_pub_key * bob_priv).to_u8_vec());
    let haB = hash256(&(bob_pub_key * alice_priv).to_u8_vec());
    println!("{}{}{}", "Asserting ".green(), "H(a*B) == H(b*A)\n".red(), format!("=>\t0x{:X} == 0x{:X}", haB, hAb).red());
    let habg = Scalar::new(hash256(&(alice_pub_key * bob_priv).to_u8_vec())) * G;
    println!("{}{}{}{}", "Calculating ".green(), "H(A*b)*G".red(), ":\n".green(), format!("=>\t{:?}", habg).red());
    let P = bob_pub_key + habg;
    println!("{}{}{}{}", "Calculating ".green(), "P = B + H(A*b)*G".red(), ":\n".green(), format!("=>\t{:?} + {:?} = {:?}", bob_pub_key, habg, P).red());
    println!("{}", "Bob's priv key can be computed as:".green());
    println!("{}", "p = b + H(A*b)".red());
    println!("{}", format!("=>\tp = 0x{:X} + 0x{:X}", bob_priv.unwrap(), hAb).red());
    let p = bob_priv + Scalar::new(hAb);
    println!("{}", format!("=>\tp = {:?}", p).red());
    println!("{}{}{}", "Let's see if ".green(), "P == p*G".red(), ":".green());
    println!("{}", format!("=>\t{:?} =? {:?} * {:?}", P, p, G).red());
    println!("{}", format!("=>\t{:?} == {:?}", P, G * p).red());
    println!("{}{}{}", "=>\tP == G * p".red(), " :=> ".green(), format!("{}", P == G * p).white().on_green());
    println!("{}", "Q.E.D".white().on_bright_green().bold());
}

fn main() {
    // type Zp = ecc_generic::types::Zp<Curve127p>;
    type Scalar = ecc_generic::types::Scalar<Curve10729p>;
    
    // dbg!(Curve127p::A, Curve127p::B, Curve127p::N);
    // if !dbg!(is_prime(dbg!(Curve127p::P))) {
    //     std::process::exit(0);
    // }
    // let c = Curve127p;
    // let n = dbg!(c.n_curve_points::<Curve127p>());
    // // if !dbg!(is_prime(dbg!(n))) {
    // //     std::process::exit(0);
    // // }
    // #[allow(non_snake_case)]
    // let G = Curve127p::generate_point();
    // let factors = dbg!(find_divisors(n));
    // #[allow(non_snake_case)]
    // let N = factors.into_iter().find(|&f| {
    //     let res = G * Scalar::new(f);
    //     println!("Testing: {f} * {:?} = {:?}", G, res);
    //     let res = res == ECpoint::Infinity;
    //     if res {
    //         println!("found ^ !");
    //     }
    //     res
    // });
    // dbg!(N);
    // match N {
    //     #[allow(non_snake_case)]
    //     Some(N) => println!("Found order of cyclic subgroup ({}) for G, {:?}", N, G),
    //     None => {
    //         println!("{:?} is not a generator?", G);
    //         println!("dumping all multiplications...");
    //         let mut x = U256::zero();
    //         while x < U256::from(2)*n {
    //             dbg!(G * dbg!(Scalar::new(x)));
    //             x += 1.into();
    //         }
    //     }
    // }
    // ::std::process::exit(0);
    // panic!();
    // let scalar = |v| Scalar::new(v);
    #[allow(non_snake_case)]
    // let G = Curve127p::generator();
    let G = Curve10729p::generate_point();
    let a = G * 5;
    let b = a / 5;
    dbg!(G, a, b);
    let inputs = [1, 2, 3, 5].map(|v| -v);//.map(scalar);
    let outputs = [8, 2];//.map(scalar);
    let fee = 1;
    dbg!(inputs.into_iter().sum::<i32>(), outputs.into_iter().sum::<i32>(), fee);
    let ins = inputs.iter().copied().map(|v| G * v).collect::<Vec<_>>();
    let mut outs = outputs.iter().copied().map(|v| G * v).collect::<Vec<_>>();
    outs.push(G * fee);
    dbg!(&ins, &outs);
    // dbg!([ins, outs].concat().into_iter().sum::<ECpoint<Curve127p>>());
    dbg!([&inputs[..], &outputs[..],&[fee]].concat().into_iter().sum::<i32>());
    #[allow(non_snake_case)]
    let H = Curve10729p::other_generator();
    dbg!(H);
    // let random = |_| thread_rng().gen_range(0..Curve10729p::P.to_string().parse().unwrap());
    let blinding_inputs = [0; 4].map(random).map(Scalar::new).map(|v| -v);//.map(scalar);
    let blinding_outputs = [0; 2].map(random).map(Scalar::new);//.map(scalar);
    let excess = [&blinding_inputs[..], &blinding_outputs[..]].concat().into_iter().sum::<Scalar>();
    let excess_in_scalar: Scalar = excess.into();
    dbg!(blinding_inputs, blinding_outputs);
    dbg!(excess);
    dbg!(excess_in_scalar);
    let bins = blinding_inputs.map(|v| H * v);
    dbg!(bins);
    dbg!(blinding_inputs.map(|v| (format!("{v}*H, or in mod form: {:?}*H", Scalar::new(v)), H * v)));
    let bouts = blinding_outputs.map(|v| H * v);
    dbg!(bouts);
    dbg!(blinding_outputs.map(|v| (format!("{v}*H, or in mod form: {:?}", Scalar::new(v)), H * v)));
    let public_excess = [&ins[..], &outs[..], &bins[..], &bouts[..]].concat().into_iter().sum::<ECpoint<Curve10729p>>();
    dbg!(public_excess);
    dbg!(H * excess);
    let h = [
        public_excess.x().unwrap().0,
        public_excess.y().unwrap().0
        ].concat()
         .into_iter()
         .map(|v| v.to_ne_bytes())
         .flatten()
         .collect::<Vec<_>>();
    let hash = hash256(&h[..]);
    dbg!(format!("0x{:X}", hash));
    dbg!(Scalar::new(hash));

    silent_payment_demo();
}