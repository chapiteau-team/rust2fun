mod common;

if_std! {
    extern crate rust2fun_laws;

    use proptest::collection::vec;
    use proptest::prelude::*;

    use rust2fun::prelude::*;
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
        fn test_invariant(fa in vec(any::<bool>(), 1..9)) {
            let fa: NEVec<_> = fa.try_into().unwrap();

            prop_assert!(invariant_identity(fa.clone()).holds());
            prop_assert!(invariant_composition(fa, print, parse, parse::<bool>, print).holds());
        }

        #[test]
        fn test_functor(fa in vec(any::<bool>(), 1..9)) {
            let fa: NEVec<_> = fa.try_into().unwrap();

            prop_assert!(covariant_identity(fa.clone()).holds());
            prop_assert!(covariant_composition(fa.clone(), print, parse::<bool>).holds());
            prop_assert!(lift_identity(fa.clone()).holds());
            prop_assert!(lift_composition(fa, print, parse::<bool>).holds());
        }

        #[test]
        fn test_semigroup(fa in vec(any::<String>(), 1..9),
                          fb in vec(any::<String>(), 1..9),
                          fc in vec(any::<String>(), 1..9)) {
            let fa: NEVec<_> = fa.try_into().unwrap();
            let fb: NEVec<_> = fb.try_into().unwrap();
            let fc: NEVec<_> = fc.try_into().unwrap();

            prop_assert!(repeat_0(fa.clone()).holds());
            prop_assert!(repeat_1(fb.clone()).holds());
            prop_assert!(semigroup_associativity(fa, fb, fc).holds());
        }

        #[test]
        fn test_semigroupal(fa in vec(any::<bool>(), 1..9),
                            fb in vec(any::<i32>(), 1..9),
                            fc in vec(any::<Result<String, u8>>(), 1..9)) {
            let fa: NEVec<_> = fa.try_into().unwrap();
            let fb: NEVec<_> = fb.try_into().unwrap();
            let fc: NEVec<_> = fc.try_into().unwrap();

            prop_assert!(semigroupal_associativity(fa, fb, fc).holds());
        }

        #[test]
        fn test_apply(fa in vec(any::<String>(), 1..9), fb in vec(any::<usize>(), 1..9)) {
            let fa: NEVec<_> = fa.try_into().unwrap();
            let fb: NEVec<_> = fb.try_into().unwrap();

            prop_assert!(map2_product_consistency(fa.clone(), fb.clone(), |a, b| a.len() == b).holds());
            prop_assert!(product_r_consistency(fa.clone(), fb.clone()).holds());
            prop_assert!(product_l_consistency(fa, fb).holds());
        }

        #[test]
        fn test_applicative(a: bool, fa in vec(any::<bool>(), 1)) {
            let fa: NEVec<_> = fa.try_into().unwrap();

            prop_assert!(applicative_identity(fa.clone()).holds());
            prop_assert!(applicative_homomorphism::<Vec<_>, _, _>(a, print).holds());
            prop_assert!(applicative_map(fa.clone(), print).holds());
            let ff= ne_vec![print; fa.len()];
            prop_assert!(ap_product_consistent(fa, ff).holds());
            prop_assert!(applicative_unit::<NEVec<_>>(a).holds());
        }

        #[test]
        fn test_flatmap(fa in vec(any::<bool>(), 1)) {
            let fa: NEVec<_> = fa.try_into().unwrap();

            prop_assert!(flat_map_associativity(fa.clone(), |x| ne_vec![print(x)], |s| ne_vec![parse::<bool>(s)]).holds());
            prop_assert!(flat_map_consistent_apply(fa.clone(), ne_vec![print; fa.len()]).holds());
            prop_assert!(m_product_consistency(fa.clone(), |x| ne_vec![print(x)]).holds());
        }

        #[test]
        fn test_monad(a: bool, fa in vec(any::<bool>(), 1..9)) {
            let fa: NEVec<_> = fa.try_into().unwrap();

            prop_assert!(monad_left_identity::<NEVec<_>, _, _>(a, |x| ne_vec![print(x)]).holds());
            prop_assert!(monad_right_identity(fa.clone()).holds());
            prop_assert!(map_flat_map_coherence(fa, print).holds());
        }
    }
}
