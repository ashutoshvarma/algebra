use ark_ec::AffineCurve;
use ark_native_boundary::serialize;
use ark_native_boundary::wrapped::{GroupAffine, GroupProjective};
use ark_pallas::{Affine, Projective};
use ark_std::{rand::Rng, test_rng};

use ark_algebra_test_templates::{curves::curve_tests, groups::group_test};
type PallasAffine = GroupAffine<Affine>;
type PallasProjective = GroupProjective<Projective>;

#[test]
fn test_projective_noncanonical_serialization() {
    serialize::tests::test_serialize_projective::<PallasProjective>();
}

#[test]
fn test_projective_curve() {
    curve_tests::<PallasProjective>();
    // sw_tests::<PallasParameters>();
}

#[test]
fn test_projective_group() {
    let mut rng = test_rng();
    let a: PallasProjective = rng.gen();
    let b: PallasProjective = rng.gen();
    group_test(a, b);
}

#[test]
fn test_generator() {
    let generator = PallasAffine::prime_subgroup_generator();
    assert!(generator.is_on_curve());
    assert!(generator.is_in_correct_subgroup_assuming_on_curve());
}
