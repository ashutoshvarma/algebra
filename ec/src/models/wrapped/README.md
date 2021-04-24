Current Status for Wrapped Models

```rust
pub use crate::wrapped::twisted_edwards_extended::{GroupAffine, GroupProjective};
```
```rust
pub use crate::wrapped::short_weierstrass_jacobian::{GroupAffine, GroupProjective};
```

## GroupAffine

### Traits

Derivative traits

- [x] `Copy`
- [x] `Clone`
- [x] `PartialEq`
- [x] `Eq`
- [x] `Debug`
- [x] `Hash`

Manual Impl

- [x] `PartialEq<GroupProjective<P>>`
- [x] `Display`
- [x] `Zeroize`
- [x] `Zero`
- [x] `Add<Self>`
- [x] `AddAssign<&'a Self>` - Since `AddAssign` is not a bound in `AffineCurve` we might impl it separately for both models
- [x] `Neg`
- [x] `ToBytes`
- [x] `FromBytes`
- [x] `Default`
- [x] `From<GroupProjective<P>>`
- [x] `CanonicalSerialize`
- [x] `CanonicalDeserialize`
- [x] `ToConstraintField<ConstraintF>`
- [x] `Sub` **(Only ED)**
- [x] `SubAssign` **(Only ED)**
- [x] `MulAssign<P::ScalarField>` **(Only ED)**
- [x] `Distribution<GroupAffine<P>> for Standard` **(Only ED)**
- [x] `core::str::FromStr` **(Only ED)**

### AffineCurve Trait

- [x] `prime_subgroup_generator()`
- [x] `from_random_bytes()`
- [x] `mul()`
- [x] `mul_by_cofactor_to_projective()`
- [x] `mul_by_cofactor_inv()`x

### Impl GroupAffine

- [x] `new()`

Need Separate wrap for both models

- [x] `scale_by_cofactor()`
- [x] `get_point_from_x()`
- [x] `is_on_curve()`
- [x] `is_in_correct_subgroup_assuming_on_curve()`
- [x] `mul_bits()`

### Macro

- [ ] `ark_ff::impl_additive_ops_from_ref` **(Only ED)**

### `mod group_impl` **(Only ED)**

- [ ] `Group` Trait

## GroupProjective

### Traits

Derivative traits

- [x] `Copy`
- [x] `Clone`
- [x] `Debug`
- [x] `Hash`
- [x] `Eq` **(Only ED)**

Manual Impl

- [x] `PartialEq<GroupAffine<P>>`
- [x] `Display`
- [x] `Eq` **(Only SW)**
- [x] `PartialEq`
- [x] `Distribution<GroupProjective<P>> for Standard`
- [x] `ToBytes`
- [x] `FromBytes`
- [x] `Default`
- [x] `Zeroize`
- [x] `Zero`
- [x] `Neg`
- [x] `Add`
- [x] `AddAssign`
- [x] `Sub`
- [x] `SubAssign`
- [x] `MulAssign`
- [x] `From<GroupAffine<P>>`
- [x] `CanonicalSerialize`
- [x] `CanonicalDeserialize`
- [x] `ToConstraintField<ConstraintF>`

### Projective

- [x] `prime_subgroup_generator()`
- [x] `is_normalized()`
- [ ] `batch_normalization()`
- [x] `double_in_place()`
- [x] `add_assign_mixed()`

### impl GroupProjective

- [x] `new()`

### Macro

- [x] `ark_ff::impl_additive_ops_from_ref`
