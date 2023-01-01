mod common;

if_std! {
    extern crate rust2fun_laws;

    use std::collections::HashMap;

    use rust2fun_laws::apply_laws::*;
    use rust2fun_laws::flatmap_laws::*;
    use rust2fun_laws::functor_laws::*;
    use rust2fun_laws::invariant_laws::*;
    use rust2fun_laws::semigroupal_laws::*;

    use crate::common::{parse, print};


    #[test]
    fn test_invariant() {
        assert!(invariant_identity(HashMap::from([(1, "id")])).holds());
        assert!(
            invariant_composition(HashMap::from([(1, 1)]), print, parse, parse::<i32>, print).holds()
        );
    }

    #[test]
    fn test_functor() {
        assert!(covariant_identity(HashMap::from([(1, 1)])).holds());
        assert!(covariant_composition(HashMap::from([(1, 1)]), print, parse::<u32>).holds());
        assert!(lift_identity(HashMap::from([(1, 1)])).holds());
        assert!(lift_composition(HashMap::from([(1, 1)]), print, parse::<i64>).holds());
    }

    #[test]
    fn test_semigroupal() {
        assert!(semigroupal_associativity(
            HashMap::from([(1, 1)]),
            HashMap::from([(1, "map".to_string())]),
            HashMap::from([(1, Ok::<_, bool>("ok"))])
        )
        .holds());
    }

    #[test]
    fn test_apply() {
        let check_length = |x: &str, l: usize| x.len() == l;

        assert!(map2_product_consistency(
            HashMap::from([(1, "str")]),
            HashMap::from([(1, 1)]),
            check_length
        )
        .holds());
        assert!(product_r_consistency(HashMap::from([(1, "str")]), HashMap::from([(1, 1)])).holds());
        assert!(product_l_consistency(HashMap::from([(1, "str")]), HashMap::from([(1, 1)])).holds());
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
