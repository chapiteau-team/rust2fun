extern crate rust2fun_laws;

use proptest::prelude::*;

use rust2fun_laws::bifunctor_laws::*;
use rust2fun_laws::monoid_laws::*;
use rust2fun_laws::semigroup_laws::*;

use crate::common::{parse, print};

mod common;

proptest! {
    #[test]
    fn test_semigroup(fa: (String, Option<String>), fb: (String, Option<String>), fc: (String, Option<String>)) {
        prop_assert!(repeat_0(fa.clone()).holds());
        prop_assert!(repeat_1(fb.clone()).holds());
        prop_assert!(semigroup_associativity(fa, fb, fc).holds());
    }

    #[test]
    fn test_monoid(fa: (String, Option<String>)) {
        prop_assert!(monoid_left_identity(fa.clone()).holds());
        prop_assert!(monoid_right_identity(fa.clone()).holds());
        prop_assert!(is_id(fa).holds());
    }

    #[test]
    fn test_bifunctor(fa: (bool, i32)) {
        prop_assert!(bifunctor_identity(fa.clone()).holds());
        prop_assert!(bifunctor_composition(fa, print, parse::<bool>, print, parse::<i32>).holds());
    }
}
