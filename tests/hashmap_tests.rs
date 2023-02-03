mod common;

if_std! {
    extern crate rust2fun_laws;

    use std::collections::HashMap;

    use proptest::prelude::*;

    use rust2fun_laws::apply_laws::*;
    use rust2fun_laws::flatmap_laws::*;
    use rust2fun_laws::functor_laws::*;
    use rust2fun_laws::invariant_laws::*;
    use rust2fun_laws::semigroupal_laws::*;

    use crate::common::{parse, print};

    proptest! {
        #[test]
        fn test_invariant(fa: HashMap::<i32, bool>) {
            assert!(invariant_identity(fa.clone()).holds());
            assert!(invariant_composition(fa, print, parse, parse::<bool>, print).holds());
        }
    }

    proptest! {
        #[test]
        fn test_functor(fa: HashMap::<i32, bool>) {
            assert!(covariant_identity(fa.clone()).holds());
            assert!(covariant_composition(fa.clone(), print, parse::<bool>).holds());
            assert!(lift_identity(fa.clone()).holds());
            assert!(lift_composition(fa, print, parse::<bool>).holds());
        }
    }

    proptest! {
        #[test]
        fn test_semigroupal(fa: HashMap::<i32, bool>, fb:HashMap::<i32, usize>, fc: HashMap<i32, Result<String, u8>>) {
            assert!(semigroupal_associativity(fa, fb, fc).holds());
        }
    }

    proptest! {
        #[test]
        fn test_apply(fa: HashMap<i32, String>, fb: HashMap<i32, usize>) {
            assert!(map2_product_consistency(fa.clone(), fb.clone(), |a, b| a.len() == b).holds());
            assert!(product_r_consistency(fa.clone(), fb.clone()).holds());
            assert!(product_l_consistency(fa, fb).holds());
        }
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
