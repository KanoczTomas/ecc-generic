
pub mod types;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::types::*;
    // use super::*;
    #[derive(Debug, PartialEq, Clone, Copy, Default)]
    struct P;
    impl GroupOrder for P {
        const P: U256 = U256([127, 0, 0, 0]);
    }
    type Zp = crate::types::Zp<P>;
    #[derive(Debug, Default, Clone, Copy, PartialEq)]
    struct HugeP;
    impl GroupOrder for HugeP {
        const P: U256 = U256::MAX;
    }
    type ZpH = crate::types::Zp<HugeP>;
    // #[derive(Debug, PartialEq, Clone, Copy, Default)]
    // struct N;
    // impl CurveOrder for N {
    //     const N: U256 = U256([127, 0, 0, 0]);
    // }
    
    #[derive(Debug, Default, Clone, Copy, PartialEq)]
    struct ElipticCurve;
    impl EC for ElipticCurve {
        const A: U256 = U256([0;4]);
        const B: U256 = U256([7,0,0,0]);
        const N: U256 = U256([127, 0, 0, 0]);
        fn order_of_cyclic_subgroup<G: GroupOrder, E: EC>(&self) -> U256 {
            todo!()
        }
        
    }
    
    type ECpoint = crate::types::ECpoint<P, ElipticCurve>;
    type Scalar = crate::types::Scalar<ElipticCurve>;
    #[test]
    fn test_zp_new(){
        let a = Zp::new(130);
        assert_eq!(a, Zp::new(3));
    }
    #[test]
    fn test_zp_addition(){
        #[derive(Debug, Default, Clone, Copy, PartialEq)]
        struct HugeP;
        impl GroupOrder for HugeP {
            // const P: U256 = U256([u64::MAX, u64::MAX, u64::MAX, u64::MAX]);
            const P: U256 = U256::MAX;
        }
        let a = Zp::new(126);
        let b = Zp::new(1);
        assert_eq!(a + b, Zp::new(0));
        let a = crate::types::Zp::<HugeP>::new(U256::MAX - 1);
        let b = crate::types::Zp::<HugeP>::new(U256::MAX - 1);
        assert_eq!((a + b).unwrap(), U256::MAX - 2);
    }
    #[test]
    fn test_zp_add_assign(){
        let mut a  = Zp::new(127);
        a += a + Zp::new(2);
        assert_eq!(Zp::new(2), a);
    }
    #[test]
    fn test_negate_non_zero(){
        let a = Zp::new(5);
        let b = -Zp::new(132);
        assert_eq!(a + b, Zp::zero());
        let a = Zp::new(10);
        let b = Zp::new(-10);
        assert_eq!(a + b, Zp::zero());
    }
    #[test]
    fn test_negate_zero(){
        assert_eq!(-Zp::zero(), Zp::zero());
    }
    #[test]
    fn test_substraction_same_value(){
        let a = Zp::new(5);
        assert_eq!(a - a, Zp::zero());
    }
    #[test]
    fn test_substract_2_different_values(){
        #[derive(Debug, Default, Clone, Copy, PartialEq)]
        struct HugeP;
        impl GroupOrder for HugeP {
            const P: U256 = U256::MAX;
        }
        let a = Zp::new(5);
        let b = Zp::new(10);
        assert_eq!(a - b, Zp::new(122));
        let a = crate::types::Zp::<HugeP>::new(3);
        let b = crate::types::Zp::<HugeP>::new(U256::MAX);
        assert_eq!(a - b, 3.into());

        
    }
    #[test]
    fn test_sub_assign(){
        let mut a =  Zp::new(256);
        a -= a;
        assert_eq!(a, Zp::zero());
    }
    #[test]
    fn test_mul(){
        #[derive(Debug, Default, Clone, Copy, PartialEq)]
        struct HugeP;
        impl GroupOrder for HugeP {
            const P: U256 = U256::MAX;
        }
        type ZpH = crate::types::Zp<HugeP>;
        let a = Zp::new(127);
        let b = Zp::new(3);
        assert_eq!(a * b, Zp::new(0));
        assert_eq!(b * Zp::new(12), Zp::new(36));
        let a = ZpH::new(U256::MAX-1); 
        let b = ZpH::new(U256::MAX-3);
        // -1 * -3 == 3
        assert_eq!(a * b, 3.into());
    }
    #[test]
    fn test_mul_assign() {
        let mut a = Zp::new(25);
        a *= a;
        assert_eq!(a, Zp::new(117));
    }
    #[test]
    fn test_division() {
        let a = Zp::new(120);
        // let b = Zp::multiplicative_inverse(Zp::new(120));
        let b = Zp::new(1) / Zp::new(120);
        let c = Zp::new(5);
        let d = -Zp::new(5);
        assert_eq!(a / a, Zp::one());
        assert_eq!(a * b, Zp::one());
        assert_eq!(c + d, Zp::zero());
    }
    #[test]
    fn test_ecpoint_new() {
        let (x, y) = (0, 1);
        // let curve = Curve::<_, P>::new().a(0).b(7).finalize();
        let ec_point = ECpoint::new(x, y);
        assert_eq!(None, ec_point);
        let (x, y) = (0, 0);
        let ec_point = ECpoint::new(x, y);
        assert_eq!(false, ec_point.is_some());
    }
    #[test]
    fn test_ecpoint_on_curve() {
        let x = 123;
        let y = 109;
        // let curve = Curve::<_, P>::new().a(0).b(7).finalize();
        let ec_point = ECpoint::new(x, y);
        assert!(ec_point.is_some());
        let ec_point = ec_point.unwrap();
        assert_eq!(ec_point.x(), Zp::new(123));
        assert_eq!(ec_point.y(), Zp::new(109));
    }
    #[test]
    fn test_ecpoint_not_on_curve() {
        let x = 123;
        let y = 108;
        // let curve = Curve::<_, P>::new().a(1).b(1).finalize();
        let ec_point = ECpoint::new(x, y);
        assert!(ec_point.is_none());
    }
    #[test]
    fn test_adding_infinities() {
        // let curve = Curve::<_, P>::new().a(0).b(7).finalize();
        let a = ECpoint::new(2, 53).unwrap();
        let b = ECpoint::Infinity;
        assert_eq!(a + b, a);
        assert_eq!(b + a, a);

        let a = ECpoint::Infinity;
        let b = ECpoint::Infinity;
        assert_eq!(a + b, ECpoint::Infinity);
    }
    #[test]
    fn test_adding_additive_inverses() {
        // https://graui.de/code/elliptic2/
        // let curve = Curve::<_, P>::new().a(0).b(7).finalize();
        let a = ECpoint::new(2, 53).unwrap();
        let b = -a;
        assert_eq!(a + b, ECpoint::Infinity);
        let c = ECpoint::new(11, 103).unwrap();
        let d = ECpoint::new(11, 24).unwrap();
        assert_eq!(c + d, ECpoint::Infinity);
    }
    #[test]
    fn test_doubleing_point() {
        let a = ECpoint::new(2, 53).unwrap();
        assert_eq!(a + a, ECpoint::new(100, 3).unwrap());
    }
    #[test]
    fn test_tripleing_point() {
        let a = ECpoint::new(1, 32).unwrap();
        assert_eq!(a + a + a, ECpoint::new(72, 16).unwrap());
    }
    #[test]
    fn test_add_2_distinct_points() {
        let a = ECpoint::new(38, 53).unwrap();
        let b = ECpoint::new(3, 65).unwrap();
        assert_eq!(a + b, ECpoint::new(32, 3).unwrap());
        let c = ECpoint::new(124, 108).unwrap();
        let d = ECpoint::new(123, 109).unwrap();
        assert_eq!(c + d, ECpoint::new(8, 30).unwrap());
    }
    #[test]
    fn test_substracting_points() {
        let a = ECpoint::new(3, 65).unwrap();
        let minus_a = -a;
        assert_eq!(a + a - a - a - a, minus_a);
    }
    #[test]
    fn skalar_multiplication_with_zero() {
        let a = ECpoint::new(38, 53).unwrap();
        assert_eq!(a * 0, ECpoint::Infinity);
    }
    #[test]
    fn skalar_multiplication_from_right() {
        #[derive(Debug, Default, Clone, Copy, PartialEq)]
        struct Secp256k1P;
        impl GroupOrder for Secp256k1P {
            //115792089237316195423570985008687907853269984665640564039457584007908834671663
            const P: U256 = U256([
                18446744069414583343,
                18446744073709551615,
                18446744073709551615,
                18446744073709551615,
            ]);
        }
        #[derive(Debug, Default, Clone, Copy, PartialEq)]
        struct CurveP256k1;
        impl EC for CurveP256k1 {
            const A: U256 = U256([0;4]);
            const B: U256 = U256([7, 0, 0, 0]);
            const N: U256 = U256([
                13822214165235122497,
                13451932020343611451,
                18446744073709551614,
                18446744073709551615,
            ]);
            //115792089237316195423570985008687907852837564279074904382605163141518161494337
            fn order_of_cyclic_subgroup<G: GroupOrder, E: EC>(&self) -> U256 {
                todo!()
            }
            
        }
        // #[derive(Debug, Default, Clone, Copy, PartialEq)]
        // struct CurveOrderSecp256k1;
        // impl CurveOrder for CurveOrderSecp256k1 {
        //     const N: U256 = U256([
        //         13822214165235122497,
        //         13451932020343611451,
        //         18446744073709551614,
        //         18446744073709551615,
        //     ]);
        // }
        type ZpH = crate::types::Zp<Secp256k1P>;
        type ECpointH = crate::types::ECpoint<Secp256k1P, CurveP256k1>;
        type ScalarH = crate::types::Scalar<CurveP256k1>;
        let a = ECpoint::new(38, 53).unwrap();
        assert_eq!(a , a * Scalar::new(1));
        assert_eq!(a + a, a * Scalar::new(2));
        assert_eq!(a + a + a, a * Scalar::new(3));
        assert_eq!(a + a + a + a, a * Scalar::new(4));
        assert_eq!(-a, a * Scalar::new(-1));
        let x = ZpH::new(U256::from_str_radix("0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798", 16).unwrap());
        let y = (x.pow(3) + ZpH::new(CurveP256k1::A) * x + ZpH::new(CurveP256k1::B)).sqrt().unwrap();
        let g = ECpointH::new(x, y).unwrap();
        let b = ScalarH::new(- 3);
        assert_eq!(g * ScalarH::one(), 
                ECpointH::new(
                    ZpH::new(U256::from_str_radix("0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798", 16).unwrap()),
                    ZpH::new(
                        U256::from_dec_str("32670510020758816978083085130507043184471273380659243275938904335757337482424").unwrap()
                    )
                ).unwrap()
            );
        assert_eq!(g * b, -g -g -g);

        //test why it is not working for these parameters, the script 
        //from https://github.com/jacksoninfosec/tonelli-shanks/blob/main/tonelli-shanks.py
        //says it should be ok, but have to test parameters for it!
    }
    #[test]
    fn skalar_multiplication_from_left() {
        let a = ECpoint::new(38, 53).unwrap();
        assert_eq!(a , Scalar::new(1) * a);
        assert_eq!(a + a, Scalar::new(2) * a);
        assert_eq!(a + a + a, Scalar::new(3) * a);
        assert_eq!(a + a + a + a, Scalar::new(4) * a);
    }
    #[test]
    fn skalar_multiplication_assign() {
        let mut a = ECpoint::new(38, 53).unwrap();
        let b = a; //copied!
        a *= Scalar::new(5);
        assert_eq!(b + b + b + b + b, a);
    }
    #[test]
    fn test_some_algebra() {
        // 2P + 3P + 5P/5P - 2P  = 4P
        let p = ECpoint::new(38, 53).unwrap();
        let left = p*2 + p*3 + p*5/5 - p*2;
        let right = p*4;
        assert_eq!(left, right);
    }
    #[test]
    fn test_curve_n_points() {
        #[derive(Debug, Clone, Copy, Default, PartialEq)]
        struct P53;
        impl GroupOrder for P53 {
            const P: U256 = U256([53, 0, 0, 0]);
        }
        let n = ElipticCurve.n_curve_points::<P, ElipticCurve>();
        assert_eq!(n, 127.into());
        let n = ElipticCurve.n_curve_points::<P53, ElipticCurve>();
        assert_eq!(n, 54.into());
    }
    #[test]
    fn test_curve_cofactor() {
        #[derive(Debug, Clone, Copy, Default, PartialEq)]
        struct P;
        impl GroupOrder for P {
            const P: U256 = U256([53, 0, 0, 0]);
        }
        let h = ElipticCurve.cofactor::<P, ElipticCurve>();
        let n = ElipticCurve.n_curve_points::<P, ElipticCurve>();
        dbg!(n);
        assert_eq!(h, 0.into());
    }
    #[test]
    fn test_raise_zp_to_power() {
        let a = 4;
        let b = Zp::new(5);
        assert_eq!(Zp::new(117), b.pow(a));
        let a = 11;
        let b = Zp::new(3);
        assert_eq!(Zp::new(109), b.pow(a));
        let a = U256::MAX / 2;
        let b = ZpH::new(34910);
        assert_eq!(ZpH::new(U256::from_dec_str("3453400382912296361574798641897645014096663415373804447676653915339892667045").unwrap()), b.pow(a));
    }
    #[test]
    fn test_quadratic_residue() {
        let a = [0, 1, 2, 4, 8, 9, 11, 13, 15, 16,
             17, 18, 19, 21, 22, 25, 26, 30, 31, 32, 34, 35,
             36, 37, 38, 41, 42, 44, 47, 49, 50, 52, 60, 61,
             62, 64, 68, 69, 70, 71, 72, 73, 74, 76, 79, 81,
             82, 84, 87, 88, 94, 98, 99, 100, 103, 104, 107,
             113, 115, 117, 120, 121, 122, 124]
             .iter()
             .map(|n| Zp::new(*n))
             .collect::<Vec<_>>();
        let b = (0..127)
            .map(|n| Zp::new(n))
            .map(|zp| (zp, zp.is_quadratic_residue()))
            .filter(|(_, is_residue)| *is_residue == true)
            .map(|(zp, _)| zp)
            .collect::<Vec<_>>();
        assert_eq!(a, b);
    }
    #[test]
    fn test_sqrt() {
        //list of all quadratic residues mod 127
        let a = [0, 1, 2, 4, 8, 9, 11, 13, 15, 16,
             17, 18, 19, 21, 22, 25, 26, 30, 31, 32, 34, 35,
             36, 37, 38, 41, 42, 44, 47, 49, 50, 52, 60, 61,
             62, 64, 68, 69, 70, 71, 72, 73, 74, 76, 79, 81,
             82, 84, 87, 88, 94, 98, 99, 100, 103, 104, 107,
             113, 115, 117, 120, 121, 122, 124]
             .iter()
             .map(|n| Zp::new(*n))
             .collect::<Vec<_>>();
        assert!(a.iter()
            .all(|z| z.sqrt().unwrap().pow(2) == *z)
        );
    }
}
