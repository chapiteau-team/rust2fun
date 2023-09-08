mod common;

if_std! {
    extern crate rust2fun_laws;

    use std::collections::HashMap;

    use proptest::prelude::*;

    use rust2fun_laws::apply_laws::*;
    use rust2fun_laws::bifunctor_laws::*;
    use rust2fun_laws::flatmap_laws::*;
    use rust2fun_laws::functor_laws::*;
    use rust2fun_laws::invariant_laws::*;
    use rust2fun_laws::monoid_laws::*;
    use rust2fun_laws::semigroup_laws::*;
    use rust2fun_laws::semigroupal_laws::*;

    use crate::common::{parse, print};

    proptest! {
        #[test]
        fn test_invariant(fa: HashMap::<i32, bool>) {
            prop_assert!(invariant_identity(fa.clone()).holds());
            prop_assert!(invariant_composition(fa, print, parse, parse::<bool>, print).holds());
        }

        #[test]
        fn test_functor(fa: HashMap::<i32, bool>) {
            prop_assert!(covariant_identity(fa.clone()).holds());
            prop_assert!(covariant_composition(fa.clone(), print, parse::<bool>).holds());
            prop_assert!(lift_identity(fa.clone()).holds());
            prop_assert!(lift_composition(fa, print, parse::<bool>).holds());
        }

        #[test]
        fn test_bifunctor(fa: HashMap::<i32, bool>) {
            prop_assert!(bifunctor_identity(fa.clone()).holds());
            prop_assert!(bifunctor_composition(fa, print, parse::<i32>, print, parse::<bool>).holds());
        }

        #[test]
        fn test_semigroupal(fa: HashMap::<i32, bool>, fb:HashMap::<i32, usize>, fc: HashMap<i32, Result<String, u8>>) {
            prop_assert!(semigroupal_associativity(fa, fb, fc).holds());
        }

        #[test]
        fn test_apply(fa: HashMap<i32, String>, fb: HashMap<i32, usize>) {
            prop_assert!(map2_product_consistency(fa.clone(), fb.clone(), |a, b| a.len() == b).holds());
            prop_assert!(product_r_consistency(fa.clone(), fb.clone()).holds());
            prop_assert!(product_l_consistency(fa, fb).holds());
        }

        #[test]
        fn test_monoid(fa: HashMap<i32, String>) {
            prop_assert!(monoid_left_identity(fa.clone()).holds());
            prop_assert!(monoid_right_identity(fa.clone()).holds());
            prop_assert!(is_id(fa).holds());
        }
    }

    #[test]
    fn test_semigroup() {
        let mut fa = HashMap::new();
        fa.insert(0, "a".to_owned());
        fa.insert(2, "a".to_owned());
        let mut fb = HashMap::new();
        fb.insert(0, "b".to_owned());
        fb.insert(1, "a".to_owned());
        let fc = HashMap::new();

        assert!(repeat_0(fa.clone()).holds());
        assert!(repeat_1(fb.clone()).holds());
        assert!(semigroup_associativity(fa, fb, fc).holds());
    }

    #[test]
    fn test_flatmap() {
        assert!(flat_map_associativity(
            HashMap::from([(1, 1)]),
            |x| HashMap::from([(1, print(x))]),
            |x| HashMap::from([(1, parse::<i32>(x))])
        )
        .holds());

        assert!(
            flat_map_consistent_apply(HashMap::from([(1, 1)]), HashMap::from([(1, print)])).holds()
        );

        assert!(
            m_product_consistency(HashMap::from([(1, 1)]), |x| HashMap::from([(1, print(x))])).holds()
        );
    }
}
