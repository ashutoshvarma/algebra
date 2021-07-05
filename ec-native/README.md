# ark-native-boundary

## Traits for Enabling Types to Cross Boundary

- [`NonCanonical(De)serialize`](https://github.com/ashutoshvarma/algebra/blob/wasm/ec/src/boundary/serialize.rs) - Fast non canonical (de)serialization for types in current implementation data needs to be serialized in order to pass boundary

- [`CurveParameters`](https://github.com/ashutoshvarma/algebra/blob/wasm/ec/src/lib.rs#L368-L370) - Since we need to know type for deserialsation at native host side, this trait provide a `ModelParameters` associated type which every curve implement and is used for dynamic type matching.

- [`CrossBoundary`](https://github.com/ashutoshvarma/algebra/blob/wasm/native-boundary/src/boundary/mod.rs#L15) - This trait introduce methods to bind native boundary with a particular type. **It is an implied trait, means its is implemented for all types that has above traits implemented**

So in order to make a type pass boundary, essentially only these traits needs to implemented `NonCanonicalSerialize`, `NonCanonicalDeserialize` and `CurveParameters`.
