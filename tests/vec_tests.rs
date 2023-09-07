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
    use rust2fun_laws::monoid_laws::*;
    use rust2fun_laws::semigroup_laws::*;
    use rust2fun_laws::semigroupal_laws::*;

    use crate::common::{parse, print};

    proptest! {
        #[test]
        fn test_invariant(fa: Vec<bool>) {
            prop_assert!(invariant_identity(fa.clone()).holds());
            prop_assert!(invariant_composition(fa, print, parse, parse::<bool>, print).holds());
        }

        #[test]
        fn test_functor(fa: Vec<bool>) {
            prop_assert!(covariant_identity(fa.clone()).holds());
            prop_assert!(covariant_composition(fa.clone(), print, parse::<bool>).holds());
            prop_assert!(lift_identity(fa.clone()).holds());
            prop_assert!(lift_composition(fa, print, parse::<bool>).holds());
        }

        #[test]
        fn test_semigroup(fa: Vec<String>, fb: Vec<String>, fc: Vec<String>) {
            prop_assert!(repeat_0(fa.clone()).holds());
            prop_assert!(repeat_1(fb.clone()).holds());
            prop_assert!(semigroup_associativity(fa, fb, fc).holds());
        }

        #[test]
        fn test_monoid(fa: Vec<String>) {
            prop_assert!(monoid_left_identity(fa.clone()).holds());
            prop_assert!(monoid_right_identity(fa.clone()).holds());
            prop_assert!(is_id(fa).holds());
        }

        #[test]
        fn test_semigroupal(fa: Vec<bool>, fb: Vec<i32>, fc: Vec<Result<String, u8>>) {
            prop_assert!(semigroupal_associativity(fa, fb, fc).holds());
        }

        #[test]
        fn test_apply(fa: Vec<String>, fb: Vec<usize>) {
            prop_assert!(map2_product_consistency(fa.clone(), fb.clone(), |a, b| a.len() == b).holds());
            prop_assert!(product_r_consistency(fa.clone(), fb.clone()).holds());
            prop_assert!(product_l_consistency(fa, fb).holds());
        }

        #[test]
        fn test_applicative(a: bool, fa in vec(any::<bool>(), 0..=1)) {
            prop_assert!(applicative_identity(fa.clone()).holds());
            prop_assert!(applicative_homomorphism::<Vec<_>, _, _>(a, print).holds());
            prop_assert!(applicative_map(fa.clone(), print).holds());
            let ff= vec![print; fa.len()];
            prop_assert!(ap_product_consistent(fa, ff).holds());
            prop_assert!(applicative_unit::<Vec<_>>(a).holds());
        }

        #[test]
        fn test_flatmap(fa in vec(any::<bool>(), 0..=1)) {
            prop_assert!(flat_map_associativity(fa.clone(), |x| vec![print(x)], |s| vec![parse::<bool>(s)]).holds());
            prop_assert!(flat_map_associativity(fa.clone(), |_| Vec::new(), |s| vec![parse::<bool>(s)]).holds());
            prop_assert!(flat_map_associativity(fa.clone(), |x| vec![print(x)], |_| Vec::<bool>::new()).holds());
            prop_assert!(flat_map_associativity(fa.clone(), |_| Vec::new(), |_: String| Vec::<bool>::new()).holds());
            prop_assert!(flat_map_consistent_apply(fa.clone(), vec![print; fa.len()]).holds());
            prop_assert!(m_product_consistency(fa.clone(), |x| vec![print(x)]).holds());
            prop_assert!(m_product_consistency(fa, |_| Vec::<String>::new()).holds());
        }

        #[test]
        fn test_monad(a: bool, fa: Vec<bool>) {
            prop_assert!(monad_left_identity::<Vec<_>, _, _>(a, |x| vec![print(x)]).holds());
            prop_assert!(monad_left_identity::<Vec<_>, _, _>(a, |_| Vec::<String>::new()).holds());
            prop_assert!(monad_right_identity(fa.clone()).holds());
            prop_assert!(map_flat_map_coherence(fa, print).holds());
        }
    }
}
