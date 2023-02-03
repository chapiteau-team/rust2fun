extern crate rust2fun_laws;

use proptest::prelude::*;

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
    fn test_invariant(fa: Option<bool>) {
        assert!(invariant_identity(fa).holds());
        assert!(invariant_composition(fa, print, parse, parse::<bool>, print).holds());
    }
}

proptest! {
    #[test]
    fn test_functor(fa: Option<bool>) {
        assert!(covariant_identity(fa).holds());
        assert!(covariant_composition(fa, print, parse::<bool>).holds());
        assert!(lift_identity(fa).holds());
        assert!(lift_composition(fa, print, parse::<bool>).holds());
    }
}

proptest! {
    #[test]
    fn test_semigroupal(fa: Option<bool>, fb: Option<i32>, fc: Option<Result<String, u8>>) {
        assert!(semigroupal_associativity(fa, fb, fc).holds());
    }
}

proptest! {
    #[test]
    fn test_apply(fa: Option<String>, fb: Option<usize>) {
        assert!(map2_product_consistency(fa.clone(), fb, |a, b| a.len() == b).holds());
        assert!(product_r_consistency(fa.clone(), fb).holds());
        assert!(product_l_consistency(fa, fb).holds());
    }
}

proptest! {
    #[test]
    fn test_applicative(fa: Option<bool>, a: bool) {
        assert!(applicative_identity(fa).holds());
        assert!(applicative_homomorphism::<Option<_>, _, _>(a, print).holds());
        assert!(applicative_map(fa, print).holds());
        assert!(ap_product_consistent(fa, Some(print)).holds());
        assert!(ap_product_consistent(fa, None::<fn(bool) -> String>).holds());
        assert!(applicative_unit::<Option<_>>(a).holds());
    }
}

proptest! {
    #[test]
    fn test_flatmap(fa: Option<bool>) {
        assert!(flat_map_associativity(fa, |x| Some(print(x)), |s| Some(parse::<bool>(s))).holds());
        assert!(flat_map_associativity(fa, |_| None, |s| Some(parse::<bool>(s))).holds());
        assert!(flat_map_associativity(fa, |x| Some(print(x)), |_| None::<bool>).holds());
        assert!(flat_map_associativity(fa, |_| None::<String>, |_| None::<bool>).holds());
        assert!(flat_map_consistent_apply(fa, Some(print)).holds());
        assert!(flat_map_consistent_apply(fa, None::<fn(bool) -> String>).holds());
        assert!(m_product_consistency(fa, |x| Some(print(x))).holds());
        assert!(m_product_consistency(fa, |_| None::<String>).holds());
    }
}

proptest! {
    #[test]
    fn test_monad(a: bool, fa: Option<bool>) {
        assert!(monad_left_identity::<Option<_>, _, _>(a, |x| Some(print(x))).holds());
        assert!(monad_left_identity::<Option<_>, _, _>(a, |_| None::<String>).holds());
        assert!(monad_right_identity(fa).holds());
        assert!(map_flat_map_coherence(fa, print).holds());
    }
}
