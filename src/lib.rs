
pub mod types;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::{types::*, utils::is_prime};
    
    #[derive(Debug, Default, Clone, Copy, PartialEq)]
    struct EllipticCurve;
    impl EC for EllipticCurve {
        const NAME: &'static str = "Curve127p";        
        const A: U256 = U256([0;4]);
        const B: U256 = U256([7,0,0,0]);
        const N: U256 = U256([127, 0, 0, 0]);
        const P: U256 = U256([127, 0, 0, 0]);
    }
    type Zp = crate::types::Zp<EllipticCurve>;   
    type ECpoint = crate::types::ECpoint<EllipticCurve>;
    type Scalar = crate::types::Scalar<EllipticCurve>;

    #[derive(Debug, Default, Clone, Copy, PartialEq)]
    struct HugeDummyCurve;
    impl EC for HugeDummyCurve {
        const NAME: &'static str = "DummyInvalidCurve";    
        const A: U256 = U256([0;4]);
        const B: U256 = U256([0;4]);
        const N: U256 = U256([0;4]);
        const P: U256 = U256::MAX;
    }
    type ZpH = crate::types::Zp<HugeDummyCurve>;

    #[derive(Debug, Default, Clone, Copy, PartialEq)]
    struct Secp256k1;
    impl EC for Secp256k1 {
        const NAME: &'static str = "Secp256k1";     
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
    type ZpSecp256k1 = crate::types::Zp<Secp256k1>;
    type ECpointSecp256k1 = crate::types::ECpoint<Secp256k1>;
    type ScalarSecp256k1 = crate::types::Scalar<Secp256k1>;
    #[test]
    fn test_zp_new(){
        let a = Zp::new(130);
        assert_eq!(a, Zp::new(3));
    }
    #[test]
    fn test_zp_addition(){

        let a = Zp::new(126);
        let b = Zp::new(1);
        assert_eq!(a + b, Zp::new(0));
        let a = ZpH::new(U256::MAX - 1);
        let b = ZpH::new(U256::MAX - 1);
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
        let a = Zp::new(5);
        let b = Zp::new(10);
        assert_eq!(a - b, Zp::new(122));
        let a = ZpH::new(3);
        let b = ZpH::new(U256::MAX);
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
        let ec_point = ECpoint::new(x, y);
        assert!(ec_point.is_none());
    }
    #[test]
    fn test_adding_infinities() {
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
        let a = ECpoint::new(38, 53).unwrap();
        assert_eq!(a , a * Scalar::new(1));
        assert_eq!(a + a, a * Scalar::new(2));
        assert_eq!(a + a + a, a * Scalar::new(3));
        assert_eq!(a + a + a + a, a * Scalar::new(4));
        assert_eq!(-a, a * Scalar::new(-1));
        let x = ZpSecp256k1::new(U256::from_str_radix("0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798", 16).unwrap());
        let y = (x.pow(3) + ZpSecp256k1::new(Secp256k1::A) * x + ZpSecp256k1::new(Secp256k1::B)).sqrt().unwrap().0;
        let g = ECpointSecp256k1::new(x, y).unwrap();
        let b = ScalarSecp256k1::new(- 3);
        assert_eq!(g * ScalarSecp256k1::one(), 
                ECpointSecp256k1::new(
                    ZpSecp256k1::new(U256::from_str_radix("0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798", 16).unwrap()),
                    ZpSecp256k1::new(
                        U256::from_dec_str("32670510020758816978083085130507043184471273380659243275938904335757337482424").unwrap()
                    )
                ).unwrap()
            );
        assert_eq!(g * b, -g -g -g);
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
            .all(|z| z.sqrt().unwrap().0.pow(2) == *z)
        );
        assert!(a.iter()
            .all(|z| z.sqrt().unwrap().1.pow(2) == *z)
        );
    }
    #[test]
    fn test_is_prime() {
        let a = is_prime(127);
        assert!(a);
    }
}
