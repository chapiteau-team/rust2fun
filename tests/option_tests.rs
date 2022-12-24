extern crate rust2fun_laws;

use rust2fun_laws::applicative_laws::*;
use rust2fun_laws::apply_laws::*;
use rust2fun_laws::functor_laws::*;
use rust2fun_laws::invariant_laws::*;
use rust2fun_laws::semigroupal_laws::*;

use crate::common::{parse, print};

mod common;

#[test]
fn test_invariant() {
    assert!(invariant_identity(None::<bool>).holds());
    assert!(invariant_identity(Some(1)).holds());

    let invariant_composition_for = |x| invariant_composition(x, print, parse, parse::<i32>, print);
    assert!(invariant_composition_for(None).holds());
    assert!(invariant_composition_for(Some(1)).holds());
}

#[test]
fn test_functor() {
    assert!(covariant_identity(None::<u32>).holds());
    assert!(covariant_identity(Some(1)).holds());

    let covariant_composition_for = |x| covariant_composition(x, print, parse::<u32>);
    assert!(covariant_composition_for(None).holds());
    assert!(covariant_composition_for(Some(1)).holds());

    assert!(lift_identity(None::<u32>).holds());
    assert!(lift_identity(Some(1)).holds());

    let lift_composition_for = |x| lift_composition(x, print, parse::<i64>);
    assert!(lift_composition_for(None).holds());
    assert!(lift_composition_for(Some(1)).holds());
}

#[test]
fn test_semigroupal() {
    assert!(
        semigroupal_associativity(None::<u32>, None::<String>, None::<Result<&str, bool>>).holds()
    );
    assert!(semigroupal_associativity(
        Some(1),
        Some("some".to_string()),
        Some(Ok::<_, bool>("ok"))
    )
    .holds());
}

#[test]
fn test_apply() {
    let check_length = |x: &str, l: usize| x.len() == l;

    assert!(map2_product_consistency(None::<&str>, None::<usize>, check_length).holds());
    assert!(map2_product_consistency(Some("some"), None::<usize>, check_length).holds());
    assert!(map2_product_consistency(Some("some"), Some(1), check_length).holds());

    assert!(product_r_consistency(None::<&str>, None::<usize>).holds());
    assert!(product_r_consistency(Some("some"), None::<usize>).holds());
    assert!(product_r_consistency(Some("some"), Some(1)).holds());

    assert!(product_l_consistency(None::<&str>, None::<usize>).holds());
    assert!(product_l_consistency(Some("some"), None::<usize>).holds());
    assert!(product_l_consistency(Some("some"), Some(1)).holds());
}

#[test]
fn test_applicative() {
    assert!(applicative_identity(None::<u32>).holds());
    assert!(applicative_identity(Some(1)).holds());

    assert!(applicative_homomorphism::<Option<_>, _, _>(1, print).holds());

    assert!(applicative_map(None, print::<i32>).holds());
    assert!(applicative_map(Some(1), print).holds());

    assert!(ap_product_consistent(None, Some(print::<i32>)).holds());
    assert!(ap_product_consistent(Some(1), Some(print)).holds());

    assert!(applicative_unit::<Option<_>>(1).holds());
}
