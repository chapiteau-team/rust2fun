mod common;

if_std! {
    extern crate rust2fun_laws;

    use std::collections::LinkedList;
    use std::iter::repeat;

    use proptest::collection::linked_list;
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
        fn test_invariant(fa: LinkedList<bool>) {
            assert!(invariant_identity(fa.clone()).holds());
            assert!(invariant_composition(fa, print, parse, parse::<bool>, print).holds());
        }
    }

    proptest! {
        #[test]
        fn test_functor(fa: LinkedList<bool>) {
            assert!(covariant_identity(fa.clone()).holds());
            assert!(covariant_composition(fa.clone(), print, parse::<bool>).holds());
            assert!(lift_identity(fa.clone()).holds());
            assert!(lift_composition(fa, print, parse::<bool>).holds());
        }
    }

    proptest! {
        #[test]
        fn test_semigroup(fa: LinkedList<String>, fb: LinkedList<String>, fc: LinkedList<String>) {
            assert!(repeat_0(fa.clone()).holds());
            assert!(repeat_1(fb.clone()).holds());
            assert!(semigroup_associativity(fa, fb, fc).holds());
        }
    }

    proptest! {
        #[test]
        fn test_semigroupal(fa: LinkedList<bool>, fb: LinkedList<i32>, fc: LinkedList<Result<String, u8>>) {
            assert!(semigroupal_associativity(fa, fb, fc).holds());
        }
    }

    proptest! {
        #[test]
        fn test_apply(fa: LinkedList<String>, fb: LinkedList<usize>) {
            assert!(map2_product_consistency(fa.clone(), fb.clone(), |a, b| a.len() == b).holds());
            assert!(product_r_consistency(fa.clone(), fb.clone()).holds());
            assert!(product_l_consistency(fa, fb).holds());
        }
    }

    proptest! {
        #[test]
        fn test_applicative(a: bool, fa in linked_list(any::<bool>(), 0..=1)) {
            let ff= repeat(print).take(fa.len()).collect::<LinkedList<_>>();

            assert!(applicative_identity(fa.clone()).holds());
            assert!(applicative_homomorphism::<Vec<_>, _, _>(a, print).holds());
            assert!(applicative_map(fa.clone(), print).holds());
            assert!(ap_product_consistent(fa, ff).holds());
            assert!(applicative_unit::<Vec<_>>(a).holds());
        }
    }

    proptest! {
        #[test]
        fn test_flatmap(fa in linked_list(any::<bool>(), 0..=1)) {
            let ff= repeat(print).take(fa.len()).collect::<LinkedList<_>>();

            assert!(flat_map_associativity(fa.clone(), |x| LinkedList::pure(print(x)), |s| LinkedList::pure(parse::<bool>(s))).holds());
            assert!(flat_map_associativity(fa.clone(), |_| LinkedList::new(), |s| LinkedList::pure(parse::<bool>(s))).holds());
            assert!(flat_map_associativity(fa.clone(), |x| LinkedList::pure(print(x)), |_| LinkedList::<bool>::new()).holds());
            assert!(flat_map_associativity(fa.clone(), |_| LinkedList::new(), |_: String| LinkedList::<bool>::new()).holds());
            assert!(flat_map_consistent_apply(fa.clone(), ff).holds());
            assert!(m_product_consistency(fa.clone(), |x| LinkedList::pure(print(x))).holds());
            assert!(m_product_consistency(fa, |_| LinkedList::<String>::new()).holds());
        }
    }

    proptest! {
        #[test]
        fn test_monad(a: bool, fa: LinkedList<bool>) {
            assert!(monad_left_identity::<LinkedList<_>, _, _>(a, |x| LinkedList::pure(print(x))).holds());
            assert!(monad_left_identity::<LinkedList<_>, _, _>(a, |_| LinkedList::<String>::new()).holds());
            assert!(monad_right_identity(fa.clone()).holds());
            assert!(map_flat_map_coherence(fa, print).holds());
        }
    }
}
