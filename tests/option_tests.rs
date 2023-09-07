extern crate rust2fun_laws;

use proptest::prelude::*;

use rust2fun_laws::applicative_laws::*;
use rust2fun_laws::apply_laws::*;
use rust2fun_laws::flatmap_laws::*;
use rust2fun_laws::functor_laws::*;
use rust2fun_laws::invariant_laws::*;
use rust2fun_laws::monad_laws::*;
use rust2fun_laws::monoid_laws::*;
use rust2fun_laws::semigroup_laws::*;
use rust2fun_laws::semigroupal_laws::*;

use crate::common::{parse, print};

mod common;

proptest! {
    #[test]
    fn test_invariant(fa: Option<bool>) {
        prop_assert!(invariant_identity(fa).holds());
        prop_assert!(invariant_composition(fa, print, parse, parse::<bool>, print).holds());
    }

    #[test]
    fn test_functor(fa: Option<bool>) {
        prop_assert!(covariant_identity(fa).holds());
        prop_assert!(covariant_composition(fa, print, parse::<bool>).holds());
        prop_assert!(lift_identity(fa).holds());
        prop_assert!(lift_composition(fa, print, parse::<bool>).holds());
    }

    #[test]
    fn test_semigroup(fa: Option<String>, fb: Option<String>, fc: Option<String>) {
        prop_assert!(repeat_0(fa.clone()).holds());
        prop_assert!(repeat_1(fb.clone()).holds());
        prop_assert!(semigroup_associativity(fa, fb, fc).holds());
    }

    #[test]
    fn test_monoid(fa: Option<String>) {
        prop_assert!(monoid_left_identity(fa.clone()).holds());
        prop_assert!(monoid_right_identity(fa.clone()).holds());
        prop_assert!(is_id(fa).holds());
    }

    #[test]
    fn test_semigroupal(fa: Option<bool>, fb: Option<i32>, fc: Option<Result<String, u8>>) {
        prop_assert!(semigroupal_associativity(fa, fb, fc).holds());
    }

    #[test]
    fn test_apply(fa: Option<String>, fb: Option<usize>) {
        prop_assert!(map2_product_consistency(fa.clone(), fb, |a, b| a.len() == b).holds());
        prop_assert!(product_r_consistency(fa.clone(), fb).holds());
        prop_assert!(product_l_consistency(fa, fb).holds());
    }

    #[test]
    fn test_applicative(fa: Option<bool>, a: bool) {
        prop_assert!(applicative_identity(fa).holds());
        prop_assert!(applicative_homomorphism::<Option<_>, _, _>(a, print).holds());
        prop_assert!(applicative_map(fa, print).holds());
        prop_assert!(ap_product_consistent(fa, Some(print)).holds());
        prop_assert!(ap_product_consistent(fa, None::<fn(bool) -> String>).holds());
        prop_assert!(applicative_unit::<Option<_>>(a).holds());
    }

    #[test]
    fn test_flatmap(fa: Option<bool>) {
        prop_assert!(flat_map_associativity(fa, |x| Some(print(x)), |s| Some(parse::<bool>(s))).holds());
        prop_assert!(flat_map_associativity(fa, |_| None, |s| Some(parse::<bool>(s))).holds());
        prop_assert!(flat_map_associativity(fa, |x| Some(print(x)), |_| None::<bool>).holds());
        prop_assert!(flat_map_associativity(fa, |_| None::<String>, |_| None::<bool>).holds());
        prop_assert!(flat_map_consistent_apply(fa, Some(print)).holds());
        prop_assert!(flat_map_consistent_apply(fa, None::<fn(bool) -> String>).holds());
        prop_assert!(m_product_consistency(fa, |x| Some(print(x))).holds());
        prop_assert!(m_product_consistency(fa, |_| None::<String>).holds());
    }

    #[test]
    fn test_monad(a: bool, fa: Option<bool>) {
        prop_assert!(monad_left_identity::<Option<_>, _, _>(a, |x| Some(print(x))).holds());
        prop_assert!(monad_left_identity::<Option<_>, _, _>(a, |_| None::<String>).holds());
        prop_assert!(monad_right_identity(fa).holds());
        prop_assert!(map_flat_map_coherence(fa, print).holds());
    }
}
