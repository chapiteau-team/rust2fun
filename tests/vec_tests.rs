mod common;

if_std! {
    extern crate rust2fun_laws;

    use rust2fun::prelude::*;

    use rust2fun_laws::applicative_laws::*;
    use rust2fun_laws::apply_laws::*;
    use rust2fun_laws::flatmap_laws::*;
    use rust2fun_laws::functor_laws::*;
    use rust2fun_laws::invariant_laws::*;
    use rust2fun_laws::monad_laws::*;
    use rust2fun_laws::semigroupal_laws::*;

    use crate::common::{parse, print};

    #[test]
    fn test_invariant() {
        assert!(invariant_identity(Vec::<bool>::default()).holds());
        assert!(invariant_identity(vec![true]).holds());
        assert!(invariant_identity(vec![1, 2, 3]).holds());

        let invariant_composition_for = |x| invariant_composition(x, print, parse, parse::<i32>, print);
        assert!(invariant_composition_for(Vec::default()).holds());
        assert!(invariant_composition_for(vec![1]).holds());
        assert!(invariant_composition_for(vec![1, 2, 3]).holds());
    }

    #[test]
    fn test_functor() {
        assert!(covariant_identity(Vec::<u32>::default()).holds());
        assert!(covariant_identity(vec![1]).holds());
        assert!(covariant_identity(vec![1, 2, 3]).holds());

        let covariant_composition_for = |x| covariant_composition(x, print, parse::<i8>);
        assert!(covariant_composition_for(Vec::default()).holds());
        assert!(covariant_composition_for(vec![1]).holds());
        assert!(covariant_composition_for(vec![1, 2, 3]).holds());

        assert!(lift_identity(Vec::<u32>::default()).holds());
        assert!(lift_identity(vec![1]).holds());
        assert!(lift_identity(vec![1, 2, 3]).holds());

        let lift_composition_for = |x| lift_composition(x, print, parse::<u8>);
        assert!(lift_composition_for(Vec::default()).holds());
        assert!(lift_composition_for(vec![1]).holds());
        assert!(lift_composition_for(vec![1, 2, 3]).holds());
    }

    #[test]
    fn test_semigroupal() {
        assert!(semigroupal_associativity(
            Vec::<u32>::default(),
            Vec::<String>::default(),
            Vec::<Result<&str, bool>>::default()
        )
        .holds());

        assert!(semigroupal_associativity(
            Vec::from([1]),
            Vec::from(["some".to_string()]),
            Vec::from([Ok::<_, bool>("ok")])
        )
        .holds());

        assert!(semigroupal_associativity(
            Vec::from([1, 2, 3]),
            Vec::from(["some".to_string(), "other".to_string()]),
            Vec::from([Ok::<_, bool>("ok"), Err::<_, bool>(false)])
        )
        .holds());
    }

    #[test]
    fn test_apply() {
        let check_length = |x: &str, l: usize| x.len() == l;

        assert!(map2_product_consistency(Vec::<&str>::default(), Vec::<usize>::default(), check_length).holds());
        assert!(map2_product_consistency(Vec::from(["some"]), Vec::from([4]), check_length).holds());
        assert!(map2_product_consistency(Vec::from(["some", "other"]), Vec::from([4, 5]), check_length).holds());

        assert!(product_r_consistency(Vec::<&str>::default(), Vec::<usize>::default()).holds());
        assert!(product_r_consistency(Vec::from(["some"]), Vec::from([4])).holds());
        assert!(product_r_consistency(Vec::from(["some", "other"]), Vec::from([4, 5])).holds());

        assert!(product_l_consistency(Vec::<&str>::default(), Vec::<usize>::default()).holds());
        assert!(product_l_consistency(Vec::from(["some"]), Vec::from([4])).holds());
        assert!(product_l_consistency(Vec::from(["some", "other"]), Vec::from([4, 5])).holds());
    }

    #[test]
    fn test_applicative() {
        assert!(applicative_identity(Vec::pure(1)).holds());
        assert!(applicative_homomorphism::<Vec<_>, _, _>(1, print).holds());
        assert!(applicative_map(Vec::pure(1), print).holds());
        assert!(ap_product_consistent(Vec::pure(1), Vec::pure(print)).holds());
        assert!(applicative_unit::<Vec<_>>(1).holds());
    }

    #[test]
    fn test_flatmap() {
        assert!(flat_map_associativity(
            Vec::pure(1),
            |x| Vec::pure(print(x)),
            |x| Vec::pure(parse::<i32>(x))
        )
        .holds());

        assert!(flat_map_consistent_apply(Vec::pure(1), Vec::pure(print)).holds());

        assert!(m_product_consistency(Vec::pure(1), |x| Vec::pure(print(x))).holds());
    }

    #[test]
    fn test_monad() {
        assert!(monad_left_identity::<Vec<_>, _, _>(1, |x| Vec::pure(print(x))).holds());
        assert!(monad_right_identity(Vec::pure(1)).holds());
        assert!(map_flat_map_coherence(Vec::pure(1), print).holds());
    }
}
