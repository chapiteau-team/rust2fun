mod common;

if_std! {
    extern crate rust2fun_laws;

    use rust2fun_laws::functor_laws::*;
    use rust2fun_laws::invariant_laws::*;

    use crate::common::{parse, print};

    #[test]
    fn test_invariant() {
        assert!(invariant_identity(Box::new("id")).holds());
        assert!(invariant_composition(Box::new(1), print, parse, parse::<i32>, print)
            .holds());
    }

    #[test]
    fn test_functor() {
        assert!(covariant_identity(Box::new(1)).holds());
        assert!(covariant_composition(Box::new(1), print, parse::<u32>).holds());
        assert!(lift_identity(Box::new(1)).holds());
        assert!(lift_composition(Box::new(1), print, parse::<i64>).holds());
    }
}
