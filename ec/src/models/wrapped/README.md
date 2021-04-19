Current Status for Wrapped Models

## GroupAffine

### Traits

Derivative traits

- [ ] `Copy`
- [ ] `Clone`
- [ ] `PartialEq`
- [ ] `Eq`
- [ ] `Debug`
- [ ] `Hash`

Manual Impl

- [ ] `PartialEq<GroupProjective<P>>`
- [ ] `Display`
- [ ] `Zeroize`
- [ ] `Zero`
- [ ] `Add<Self>`
- [ ] `AddAssign<&'a Self>`
- [ ] `Neg`
- [ ] `ToBytes`
- [ ] `FromBytes`
- [ ] `Default`
- [ ] `From<GroupProjective<P>>`
- [ ] `CanonicalSerialize`
- [ ] `CanonicalDeserialize`
- [ ] `ToConstraintField<ConstraintF>`
- [ ] `Sub` **(Only ED)**
- [ ] `SubAssign` **(Only ED)**
- [ ] `MulAssign` **(Only ED)**
- [ ] `Distribution<GroupAffine<P>> for Standard` **(Only ED)**

### AffineCurve Trait

- [ ] `prime_subgroup_generator()`
- [ ] `from_random_bytes()`
- [ ] `mul()`
- [ ] `mul_by_cofactor_to_projective()`
- [ ] `mul_by_cofactor_inv()`

### Impl GroupAffine

- [ ] `new()`
- [ ] `scale_by_cofactor()`
- [ ] `get_point_from_x()`
- [ ] `is_on_curve()`
- [ ] `is_in_correct_subgroup_assuming_on_curve()`
- [ ] `mul_bits()` **(Only ED)**

### Macro

- [ ] `ark_ff::impl_additive_ops_from_ref` **(Only ED)**

### `mod group_impl` **(Only ED)**

- [ ] `Group` Trait

## GroupProjective

### Traits

Derivative traits

- [ ] `Copy`
- [ ] `Clone`
- [ ] `Debug`
- [ ] `Hash`
- [ ] `Eq` **(Only ED)**

Manual Impl

- [ ] `PartialEq<GroupAffine<P>>`
- [ ] `Display`
- [ ] `Eq` **(Only SW)**
- [ ] `PartialEq`
- [ ] `Distribution<GroupProjective<P>> for Standard`
- [ ] `ToBytes`
- [ ] `FromBytes`
- [ ] `Default`
- [ ] `Zeroize`
- [ ] `Zero`
- [ ] `Neg`
- [ ] `Add`
- [ ] `AddAssign`
- [ ] `Sub`
- [ ] `SubAssign`
- [ ] `MulAssign`
- [ ] `From<GroupAffine<P>>`
- [ ] `CanonicalSerialize`
- [ ] `CanonicalDeserialize`
- [ ] `ToConstraintField<ConstraintF>`
- [ ] `core::str::FromStr` **(Only ED)**

### Projective

- [ ] `prime_subgroup_generator()`
- [ ] `is_normalized()`
- [ ] `batch_normalization()`
- [ ] `double_in_place()`
- [ ] `add_assign_mixed()`

### impl GroupProjective

- [ ] `new()`

### Macro

- [ ] `ark_ff::impl_additive_ops_from_ref`
