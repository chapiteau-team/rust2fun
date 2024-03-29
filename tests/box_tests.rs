mod common;

if_std! {
    extern crate rust2fun_laws;

    use rust2fun_laws::applicative_laws::*;
    use rust2fun_laws::apply_laws::*;
    use rust2fun_laws::flatmap_laws::*;
    use rust2fun_laws::functor_laws::*;
    use rust2fun_laws::invariant_laws::*;
    use rust2fun_laws::monad_laws::*;
    use rust2fun_laws::semigroup_laws::*;
    use rust2fun_laws::semigroupal_laws::*;

    use crate::common::{parse, print};

    #[test]
    fn test_invariant() {
        assert!(invariant_identity(Box::new("id")).holds());
        assert!(invariant_composition(Box::new(1), print, parse, parse::<i32>, print).holds());
    }

    #[test]
    fn test_functor() {
        assert!(covariant_identity(Box::new(1)).holds());
        assert!(covariant_composition(Box::new(1), print, parse::<u32>).holds());
        assert!(lift_identity(Box::new(1)).holds());
        assert!(lift_composition(Box::new(1), print, parse::<i64>).holds());
    }

    #[test]
    fn test_semigroup() {
        assert!(repeat_0(Box::new(1)).holds());
        assert!(repeat_1(Box::new(1)).holds());
        assert!(semigroup_associativity(Box::new(1), Box::new(2), Box::new(3)).holds());
    }

    #[test]
    fn test_semigroupal() {
        assert!(semigroupal_associativity(
            Box::new(1),
            Box::new("box".to_string()),
            Box::new(Ok::<_, bool>("ok"))
        )
        .holds());
    }

    #[test]
    fn test_apply() {
        let check_length = |x: &str, l: usize| x.len() == l;

        assert!(map2_product_consistency(Box::new("str"), Box::new(1), check_length).holds());
        assert!(product_r_consistency(Box::new("str"), Box::new(1)).holds());
        assert!(product_l_consistency(Box::new("str"), Box::new(1)).holds());
    }

    #[test]
    fn test_applicative() {
        assert!(applicative_identity(Box::new(1)).holds());
        assert!(applicative_homomorphism::<Box<_>, _, _>(1, print).holds());
        assert!(applicative_map(Box::new(1), print).holds());
        assert!(ap_product_consistent(Box::new(1), Box::new(print)).holds());
        assert!(applicative_unit::<Box<_>>(1).holds());
    }

    #[test]
    fn test_flatmap() {
        assert!(flat_map_associativity(Box::new(1), |x| Box::new(x + 1), |x| Box::new(x + 2)).holds());
        assert!(flat_map_consistent_apply(Box::new(1), Box::new(print)).holds());
        assert!(m_product_consistency(Box::new(1), |x| Box::new(x + 1)).holds());
    }

    #[test]
    fn test_monad() {
        assert!(monad_left_identity::<Box<_>, _, _>(1, |x| Box::new(x + 1)).holds());
        assert!(monad_right_identity(Box::new(1)).holds());
        assert!(map_flat_map_coherence(Box::new(1), |x| x + 1).holds());
    }
}
