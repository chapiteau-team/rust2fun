mod common;

if_std! {
    extern crate rust2fun_laws;

    use std::collections::LinkedList;

    use rust2fun_laws::functor_laws::*;
    use rust2fun_laws::invariant_laws::*;

    use crate::common::{parse, print};

    #[test]
    fn test_invariant() {
        assert!(invariant_identity(LinkedList::<()>::default()).holds());
        assert!(invariant_identity(LinkedList::from([true])).holds());
        assert!(invariant_identity(LinkedList::from([1, 2, 3])).holds());

        let invariant_composition_for = |x| invariant_composition(x,
                                                                  print,
                                                                  parse,
                                                                  parse::<i32>,
                                                                  print);
        assert!(invariant_composition_for(LinkedList::default()).holds());
        assert!(invariant_composition_for(LinkedList::from([1])).holds());
        assert!(invariant_composition_for(LinkedList::from([1, 2, 3])).holds());
    }

    #[test]
    fn test_functor() {
        assert!(covariant_identity(LinkedList::<u32>::default()).holds());
        assert!(covariant_identity(LinkedList::from([1])).holds());
        assert!(covariant_identity(LinkedList::from([1, 2, 3])).holds());

        let covariant_composition_for = |x| covariant_composition(x,
                                                                  print,
                                                                  parse::<i8>);
        assert!(covariant_composition_for(LinkedList::default()).holds());
        assert!(covariant_composition_for(LinkedList::from([1])).holds());
        assert!(covariant_composition_for(LinkedList::from([1, 2, 3])).holds());

        assert!(lift_identity(LinkedList::<u32>::default()).holds());
        assert!(lift_identity(LinkedList::from([1])).holds());
        assert!(lift_identity(LinkedList::from([1, 2, 3])).holds());

        let lift_composition_for = |x| lift_composition(x,
                                                        print,
                                                        parse::<u8>);
        assert!(lift_composition_for(LinkedList::default()).holds());
        assert!(lift_composition_for(LinkedList::from([1])).holds());
        assert!(lift_composition_for(LinkedList::from([1, 2, 3])).holds());
    }
}