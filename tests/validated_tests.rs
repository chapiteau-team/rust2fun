extern crate rust2fun_laws;

use proptest::proptest;

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

        assert!(invariant_identity(fa).holds());
        assert!(invariant_composition(fa, print, parse, parse::<bool>, print).holds());
    }
}

proptest! {
    #[test]
    fn test_functor(fa: Result<bool, i32>) {
        let fa: Validated<_, i32> = fa.into();

        assert!(covariant_identity(fa).holds());
        assert!(covariant_composition(fa, print, parse::<bool>).holds());
        assert!(lift_identity(fa).holds());
        assert!(lift_composition(fa, print, parse::<bool>).holds());
    }
}

proptest! {
    #[test]
    fn test_semigroup(fa: Result<(), String>, fb: Result<(), String>, fc: Result<(), String>) {
        let fa: Validated<_, String> = fa.into();
        let fb: Validated<_,_> = fb.into();
        let fc: Validated<_,_> = fc.into();

        assert!(repeat_0(fa.clone()).holds());
        assert!(repeat_1(fb.clone()).holds());
        assert!(semigroup_associativity(fa, fb, fc).holds());
    }
}

proptest! {
    #[test]
    fn test_semigroupal(fa: Result<bool, String>, fb: Result<String, String>, fc: Result<Option<String>, String>) {
        let fa: Validated<_, String> = fa.into();
        let fb: Validated<_,_> = fb.into();
        let fc: Validated<_,_> = fc.into();

        assert!(semigroupal_associativity(fa, fb, fc).holds());
    }
}

proptest! {
    #[test]
    fn test_apply(fa: Result<String, String>, fb: Result<usize, String>) {
        let fa: Validated<_, String> = fa.into();
        let fb: Validated<_,_> = fb.into();

        assert!(map2_product_consistency(fa.clone(), fb.clone(), |a, b| a.len() == b).holds());
        assert!(product_r_consistency(fa.clone(), fb.clone()).holds());
        assert!(product_l_consistency(fa, fb).holds());
    }
}

proptest! {
    #[test]
    fn test_applicative(fa: Result<bool, i32>, a: bool) {
        let fa: Validated<_,_> = fa.into();

        assert!(applicative_identity(fa).holds());
        assert!(applicative_homomorphism::<Option<_>, _, _>(a, print).holds());
        assert!(applicative_map(fa, print).holds());
        assert!(ap_product_consistent(fa, Valid(print)).holds());
        assert!(ap_product_consistent(fa, Invalid::<fn(bool) -> String, _>(-1)).holds());
        assert!(applicative_unit::<Option<_>>(a).holds());
    }
}
