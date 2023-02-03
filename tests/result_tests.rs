extern crate rust2fun_laws;

use proptest::proptest;

use rust2fun_laws::applicative_laws::*;
use rust2fun_laws::apply_laws::*;
use rust2fun_laws::flatmap_laws::*;
use rust2fun_laws::functor_laws::*;
use rust2fun_laws::invariant_laws::*;
use rust2fun_laws::monad_laws::*;
use rust2fun_laws::semigroupal_laws::*;

use crate::common::{parse, print};

mod common;

proptest! {
    #[test]
    fn test_invariant(fa: Result<bool, i32>) {
        assert!(invariant_identity(fa).holds());
        assert!(invariant_composition(fa, print, parse, parse::<bool>, print).holds());
    }
}

proptest! {
    #[test]
    fn test_functor(fa: Result<bool, i32>) {
        assert!(covariant_identity(fa).holds());
        assert!(covariant_composition(fa, print, parse::<bool>).holds());
        assert!(lift_identity(fa).holds());
        assert!(lift_composition(fa, print, parse::<bool>).holds());
    }
}

proptest! {
    #[test]
    fn test_semigroupal(fa: Result<bool, i32>, fb: Result<String, i32>, fc: Result<Option<String>, i32>) {
        assert!(semigroupal_associativity(fa, fb, fc).holds());
    }
}

proptest! {
    #[test]
    fn test_apply(fa: Result<String, i32>, fb: Result<usize, i32>) {
        assert!(map2_product_consistency(fa.clone(), fb, |a, b| a.len() == b).holds());
        assert!(product_r_consistency(fa.clone(), fb).holds());
        assert!(product_l_consistency(fa, fb).holds());
    }
}

proptest! {
    #[test]
    fn test_applicative(fa: Result<bool, i32>, a: bool) {
        assert!(applicative_identity(fa).holds());
        assert!(applicative_homomorphism::<Option<_>, _, _>(a, print).holds());
        assert!(applicative_map(fa, print).holds());
        assert!(ap_product_consistent(fa, Ok(print)).holds());
        assert!(ap_product_consistent(fa, Err::<fn(bool) -> String, _>(-1)).holds());
        assert!(applicative_unit::<Option<_>>(a).holds());
    }
}

proptest! {
    #[test]
    fn test_flatmap(fa: Result<bool, i32>) {
        assert!(flat_map_associativity(fa, |x| Ok(print(x)), |s| Ok(parse::<bool>(s))).holds());
        assert!(flat_map_associativity(fa, |_| Err(-1), |s| Ok(parse::<bool>(s))).holds());
        assert!(flat_map_associativity(fa, |x| Ok(print(x)), |_| Err::<bool, _>(-1)).holds());
        assert!(flat_map_associativity(fa, |_| Err::<String, _>(-1), |_| Err::<bool, _>(-1)).holds());
        assert!(flat_map_consistent_apply(fa, Ok(print)).holds());
        assert!(flat_map_consistent_apply(fa, Err::<fn(bool)-> String, _>(-1)).holds());
        assert!(m_product_consistency(fa, |x| Ok(print(x))).holds());
        assert!(m_product_consistency(fa, |_| Err::<String, _>(-1)).holds());
    }
}

proptest! {
    #[test]
    fn test_monad(a: bool, fa: Result<bool, i32>) {
        assert!(monad_left_identity::<Result<_, _>, _, _>(a, |x| Ok::<_, i32>(print(x))).holds());
        assert!(monad_left_identity::<Result<_, _>, _, _>(a, |_| Err::<String, _>(-1)).holds());
        assert!(monad_right_identity(fa).holds());
        assert!(map_flat_map_coherence(fa, print).holds());
    }
}
