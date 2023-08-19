mod common;

if_std! {
    extern crate rust2fun_laws;

    use proptest::collection::vec;
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

    proptest! {
        #[test]
        fn test_invariant(fa: Vec<bool>) {
            assert!(invariant_identity(fa.clone()).holds());
            assert!(invariant_composition(fa, print, parse, parse::<bool>, print).holds());
        }
    }

    proptest! {
        #[test]
        fn test_functor(fa: Vec<bool>) {
            assert!(covariant_identity(fa.clone()).holds());
            assert!(covariant_composition(fa.clone(), print, parse::<bool>).holds());
            assert!(lift_identity(fa.clone()).holds());
            assert!(lift_composition(fa, print, parse::<bool>).holds());
        }
    }

    proptest! {
        #[test]
        fn test_semigroup(fa: Vec<String>, fb: Vec<String>, fc: Vec<String>) {
            assert!(repeat_0(fa.clone()).holds());
            assert!(repeat_1(fb.clone()).holds());
            assert!(semigroup_associativity(fa, fb, fc).holds());
        }
    }

    proptest! {
        #[test]
        fn test_semigroupal(fa: Vec<bool>, fb: Vec<i32>, fc: Vec<Result<String, u8>>) {
            assert!(semigroupal_associativity(fa, fb, fc).holds());
        }
    }

    proptest! {
        #[test]
        fn test_apply(fa: Vec<String>, fb: Vec<usize>) {
            assert!(map2_product_consistency(fa.clone(), fb.clone(), |a, b| a.len() == b).holds());
            assert!(product_r_consistency(fa.clone(), fb.clone()).holds());
            assert!(product_l_consistency(fa, fb).holds());
        }
    }

    proptest! {
        #[test]
        fn test_applicative(a: bool, fa in vec(any::<bool>(), 0..=1)) {
            assert!(applicative_identity(fa.clone()).holds());
            assert!(applicative_homomorphism::<Vec<_>, _, _>(a, print).holds());
            assert!(applicative_map(fa.clone(), print).holds());
            let ff= vec![print; fa.len()];
            assert!(ap_product_consistent(fa, ff).holds());
            assert!(applicative_unit::<Vec<_>>(a).holds());
        }
    }

    proptest! {
        #[test]
        fn test_flatmap(fa in vec(any::<bool>(), 0..=1)) {
            assert!(flat_map_associativity(fa.clone(), |x| vec![print(x)], |s| vec![parse::<bool>(s)]).holds());
            assert!(flat_map_associativity(fa.clone(), |_| Vec::new(), |s| vec![parse::<bool>(s)]).holds());
            assert!(flat_map_associativity(fa.clone(), |x| vec![print(x)], |_| Vec::<bool>::new()).holds());
            assert!(flat_map_associativity(fa.clone(), |_| Vec::new(), |_: String| Vec::<bool>::new()).holds());
            assert!(flat_map_consistent_apply(fa.clone(), vec![print; fa.len()]).holds());
            assert!(m_product_consistency(fa.clone(), |x| vec![print(x)]).holds());
            assert!(m_product_consistency(fa, |_| Vec::<String>::new()).holds());
        }
    }

    proptest! {
        #[test]
        fn test_monad(a: bool, fa: Vec<bool>) {
            assert!(monad_left_identity::<Vec<_>, _, _>(a, |x| vec![print(x)]).holds());
            assert!(monad_left_identity::<Vec<_>, _, _>(a, |_| Vec::<String>::new()).holds());
            assert!(monad_right_identity(fa.clone()).holds());
            assert!(map_flat_map_coherence(fa, print).holds());
        }
    }
}
