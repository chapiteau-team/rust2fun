extern crate rust2fun_laws;

use std::marker::PhantomData;

use rust2fun_laws::applicative_laws::*;
use rust2fun_laws::apply_laws::*;
use rust2fun_laws::contravariant_laws::*;
use rust2fun_laws::functor_laws::*;
use rust2fun_laws::invariant_laws::*;
use rust2fun_laws::semigroupal_laws::*;

use crate::common::{parse, print};

mod common;

#[test]
fn test_invariant() {
    assert!(invariant_identity(PhantomData::<bool>).holds());

    let invariant_composition_for = |x| invariant_composition(x, print, parse, parse::<i32>, print);
    assert!(invariant_composition_for(PhantomData::<i32>).holds());
}

#[test]
fn test_functor() {
    assert!(covariant_identity(PhantomData::<u32>).holds());

    let covariant_composition_for = |x| covariant_composition(x, print, parse::<u32>);
    assert!(covariant_composition_for(PhantomData::<i32>).holds());

    assert!(lift_identity(PhantomData::<u32>).holds());

    let lift_composition_for = |x| lift_composition(x, print, parse::<i64>);
    assert!(lift_composition_for(PhantomData::<i32>).holds());
}

#[test]
fn test_contravariant() {
    assert!(contravariant_identity(PhantomData::<u32>).holds());

    let covariant_composition_for = |x| contravariant_composition(x, parse::<i32>, print::<u32>);
    assert!(covariant_composition_for(PhantomData::<i32>).holds());

    assert!(lift_contravariant_identity(PhantomData::<u32>).holds());

    let lift_contravariant_composition_for =
        |x| lift_contravariant_composition(x, parse::<i32>, print::<u32>);
    assert!(lift_contravariant_composition_for(PhantomData::<i32>).holds());
}

#[test]
fn test_semigroupal() {
    assert!(
        semigroupal_associativity(PhantomData::<u32>, PhantomData::<u32>, PhantomData::<u32>)
            .holds()
    );
}

#[test]
fn test_apply() {
    let check_length = |x: &str, l: usize| x.len() == l;

    assert!(
        map2_product_consistency(PhantomData::<&str>, PhantomData::<usize>, check_length).holds()
    );
    assert!(product_r_consistency(PhantomData::<u32>, PhantomData::<u32>).holds());
    assert!(product_l_consistency(PhantomData::<u32>, PhantomData::<u32>).holds());
}

#[test]
fn test_applicative() {
    assert!(applicative_identity(PhantomData::<u32>).holds());
    assert!(applicative_homomorphism::<PhantomData<_>, _, _>(1, print).holds());
    assert!(applicative_map(PhantomData::<i32>, print).holds());
    assert!(ap_product_consistent(PhantomData::<i32>, PhantomData::<fn(i32) -> String>).holds());
    assert!(applicative_unit::<PhantomData<_>>(1).holds());
}
