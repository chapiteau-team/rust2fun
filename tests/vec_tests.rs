mod common;

if_std! {
    extern crate rust2fun_laws;

    use rust2fun_laws::functor_laws::*;
    use rust2fun_laws::invariant_laws::*;

    use crate::common::{parse, print};

    #[test]
    fn test_invariant() {
        assert!(invariant_identity(Vec::<bool>::default()).holds());
        assert!(invariant_identity(vec![true]).holds());
        assert!(invariant_identity(vec![1, 2, 3]).holds());

        let invariant_composition_for = |x| invariant_composition(x,
                                                                  print,
                                                                  parse,
                                                                  parse::<i32>,
                                                                  print);
        assert!(invariant_composition_for(Vec::default()).holds());
        assert!(invariant_composition_for(vec![1]).holds());
        assert!(invariant_composition_for(vec![1, 2, 3]).holds());
    }

    #[test]
    fn test_functor() {
        assert!(covariant_identity(Vec::<u32>::default()).holds());
        assert!(covariant_identity(vec![1]).holds());
        assert!(covariant_identity(vec![1, 2, 3]).holds());

        let covariant_composition_for = |x| covariant_composition(x,
                                                                  print,
                                                                  parse::<i8>);
        assert!(covariant_composition_for(Vec::default()).holds());
        assert!(covariant_composition_for(vec![1]).holds());
        assert!(covariant_composition_for(vec![1, 2, 3]).holds());

        assert!(lift_identity(Vec::<u32>::default()).holds());
        assert!(lift_identity(vec![1]).holds());
        assert!(lift_identity(vec![1, 2, 3]).holds());

        let lift_composition_for = |x| lift_composition(x,
                                                        print,
                                                        parse::<u8>);
        assert!(lift_composition_for(Vec::default()).holds());
        assert!(lift_composition_for(vec![1]).holds());
        assert!(lift_composition_for(vec![1, 2, 3]).holds());
    }
}