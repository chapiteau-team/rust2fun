extern crate rust2fun_laws;

use proptest::prelude::*;

use rust2fun::prelude::*;
use rust2fun_laws::applicative_laws::*;
use rust2fun_laws::apply_laws::*;
use rust2fun_laws::functor_laws::*;
use rust2fun_laws::invariant_laws::*;
use rust2fun_laws::semigroup_laws::*;
use rust2fun_laws::semigroupal_laws::*;

use crate::common::{parse, print};

mod common;

proptest! {
    #[test]
    fn test_invariant(fa: Result<bool, i32>) {
        let fa: Validated<_, i32> = fa.into();

        prop_assert!(invariant_identity(fa).holds());
        prop_assert!(invariant_composition(fa, print, parse, parse::<bool>, print).holds());
    }

    #[test]
    fn test_functor(fa: Result<bool, i32>) {
        let fa: Validated<_, i32> = fa.into();

        prop_assert!(covariant_identity(fa).holds());
        prop_assert!(covariant_composition(fa, print, parse::<bool>).holds());
        prop_assert!(lift_identity(fa).holds());
        prop_assert!(lift_composition(fa, print, parse::<bool>).holds());
    }

    #[test]
    fn test_semigroup(fa: Result<(), String>, fb: Result<(), String>, fc: Result<(), String>) {
        let fa: Validated<_, String> = fa.into();
        let fb: Validated<_,_> = fb.into();
        let fc: Validated<_,_> = fc.into();

        prop_assert!(repeat_0(fa.clone()).holds());
        prop_assert!(repeat_1(fb.clone()).holds());
        prop_assert!(semigroup_associativity(fa, fb, fc).holds());
    }

    #[test]
    fn test_semigroupal(fa: Result<bool, String>, fb: Result<String, String>, fc: Result<Option<String>, String>) {
        let fa: Validated<_, String> = fa.into();
        let fb: Validated<_,_> = fb.into();
        let fc: Validated<_,_> = fc.into();

        prop_assert!(semigroupal_associativity(fa, fb, fc).holds());
    }

    #[test]
    fn test_apply(fa: Result<String, String>, fb: Result<usize, String>) {
        let fa: Validated<_, String> = fa.into();
        let fb: Validated<_,_> = fb.into();

        prop_assert!(map2_product_consistency(fa.clone(), fb.clone(), |a, b| a.len() == b).holds());
        prop_assert!(product_r_consistency(fa.clone(), fb.clone()).holds());
        prop_assert!(product_l_consistency(fa, fb).holds());
    }

    #[test]
    fn test_applicative(fa: Result<bool, i32>, a: bool) {
        let fa: Validated<_,_> = fa.into();

        prop_assert!(applicative_identity(fa).holds());
        prop_assert!(applicative_homomorphism::<Option<_>, _, _>(a, print).holds());
        prop_assert!(applicative_map(fa, print).holds());
        prop_assert!(ap_product_consistent(fa, Valid(print)).holds());
        prop_assert!(ap_product_consistent(fa, Invalid::<fn(bool) -> String, _>(-1)).holds());
        prop_assert!(applicative_unit::<Option<_>>(a).holds());
    }
}
