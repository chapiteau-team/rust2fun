extern crate rust2fun_laws;

use proptest::prelude::*;

use rust2fun_laws::applicative_laws::*;
use rust2fun_laws::apply_laws::*;
use rust2fun_laws::flatmap_laws::*;
use rust2fun_laws::functor_laws::*;
use rust2fun_laws::invariant_laws::*;
use rust2fun_laws::monad_laws::*;
use rust2fun_laws::semigroup_laws::*;
use rust2fun_laws::semigroupal_laws::*;

use crate::common::{parse, print};

mod common;

proptest! {
    #[test]
    fn test_invariant(fa: Result<bool, i32>) {
        prop_assert!(invariant_identity(fa).holds());
        prop_assert!(invariant_composition(fa, print, parse, parse::<bool>, print).holds());
    }

    #[test]
    fn test_functor(fa: Result<bool, i32>) {
        prop_assert!(covariant_identity(fa).holds());
        prop_assert!(covariant_composition(fa, print, parse::<bool>).holds());
        prop_assert!(lift_identity(fa).holds());
        prop_assert!(lift_composition(fa, print, parse::<bool>).holds());
    }

    #[test]
    fn test_semigroup(fa: Result<(), u8>, fb: Result<(), u8>, fc: Result<(), u8>) {
        prop_assert!(repeat_0(fa.clone()).holds());
        prop_assert!(repeat_1(fb.clone()).holds());
        prop_assert!(semigroup_associativity(fa, fb, fc).holds());
    }

    #[test]
    fn test_semigroupal(fa: Result<bool, i32>, fb: Result<String, i32>, fc: Result<Option<String>, i32>) {
        prop_assert!(semigroupal_associativity(fa, fb, fc).holds());
    }

    #[test]
    fn test_apply(fa: Result<String, i32>, fb: Result<usize, i32>) {
        prop_assert!(map2_product_consistency(fa.clone(), fb, |a, b| a.len() == b).holds());
        prop_assert!(product_r_consistency(fa.clone(), fb).holds());
        prop_assert!(product_l_consistency(fa, fb).holds());
    }

    #[test]
    fn test_applicative(fa: Result<bool, i32>, a: bool) {
        prop_assert!(applicative_identity(fa).holds());
        prop_assert!(applicative_homomorphism::<Option<_>, _, _>(a, print).holds());
        prop_assert!(applicative_map(fa, print).holds());
        prop_assert!(ap_product_consistent(fa, Ok(print)).holds());
        prop_assert!(ap_product_consistent(fa, Err::<fn(bool) -> String, _>(-1)).holds());
        prop_assert!(applicative_unit::<Option<_>>(a).holds());
    }

    #[test]
    fn test_flatmap(fa: Result<bool, i32>) {
        prop_assert!(flat_map_associativity(fa, |x| Ok(print(x)), |s| Ok(parse::<bool>(s))).holds());
        prop_assert!(flat_map_associativity(fa, |_| Err(-1), |s| Ok(parse::<bool>(s))).holds());
        prop_assert!(flat_map_associativity(fa, |x| Ok(print(x)), |_| Err::<bool, _>(-1)).holds());
        prop_assert!(flat_map_associativity(fa, |_| Err::<String, _>(-1), |_| Err::<bool, _>(-1)).holds());
        prop_assert!(flat_map_consistent_apply(fa, Ok(print)).holds());
        prop_assert!(flat_map_consistent_apply(fa, Err::<fn(bool)-> String, _>(-1)).holds());
        prop_assert!(m_product_consistency(fa, |x| Ok(print(x))).holds());
        prop_assert!(m_product_consistency(fa, |_| Err::<String, _>(-1)).holds());
    }

    #[test]
    fn test_monad(a: bool, fa: Result<bool, i32>) {
        prop_assert!(monad_left_identity::<Result<_, _>, _, _>(a, |x| Ok::<_, i32>(print(x))).holds());
        prop_assert!(monad_left_identity::<Result<_, _>, _, _>(a, |_| Err::<String, _>(-1)).holds());
        prop_assert!(monad_right_identity(fa).holds());
        prop_assert!(map_flat_map_coherence(fa, print).holds());
    }
}
