
pub mod types;

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
    struct ElipticCurve;
    impl EC for ElipticCurve {
        const A: i64 = 0;
        const B: i64 = 7;
        
        fn generator<G: GroupOrder, E: EC>(&self) -> crate::types::ECpoint<G, E> {
            todo!()
        }
        
        fn n_curve_points<G: GroupOrder, E: EC>(&self) -> U256 {
            todo!()
        }
        
        fn cofactor<G: GroupOrder, E: EC>(&self) -> U256 {
            todo!()
        }
        
        fn order_of_cyclic_subgroup<G: GroupOrder, E: EC>(&self) -> U256 {
            todo!()
        }
    }
    type ECpoint = crate::types::ECpoint<P, ElipticCurve>;
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
        assert_eq!(a - b, Zp::new(122))
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
        assert_eq!(true, ec_point.is_some());
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
    fn skalar_multiplication_from_right() {
        let a = ECpoint::new(38, 53).unwrap();
        assert_eq!(a , a * Zp::new(1));
        assert_eq!(a + a, a * Zp::new(2));
        assert_eq!(a + a + a, a * Zp::new(3));
        assert_eq!(a + a + a + a, a * Zp::new(4));
        assert_eq!(-a, a * Zp::new(-1));
    }
    #[test]
    fn skalar_multiplication_from_left() {
        let a = ECpoint::new(38, 53).unwrap();
        assert_eq!(a , Zp::new(1) * a);
        assert_eq!(a + a, Zp::new(2) * a);
        assert_eq!(a + a + a, Zp::new(3) * a);
        assert_eq!(a + a + a + a, Zp::new(4) * a);
    }
    #[test]
    fn skalar_multiplication_assign() {
        let mut a = ECpoint::new(38, 53).unwrap();
        let b = a; //copied!
        a *= Zp::new(5);
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


}
