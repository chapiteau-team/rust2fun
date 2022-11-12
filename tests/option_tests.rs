extern crate rust2fun_laws;

use rust2fun_laws::invariant_laws::*;

#[test]
fn test_invariant() {
    assert!(invariant_identity(None::<bool>).holds());
    assert!(invariant_identity(Some(1)).holds());

    let invariant_composition_for = |x| invariant_composition(
        x,
        |x: i32| x.to_string(),
        |x: String| x.parse().unwrap(),
        |x: String| x.parse().unwrap(),
        |x: i32| x.to_string());
    assert!(invariant_composition_for(Some(1)).holds());
    assert!(invariant_composition_for(None).holds());
}