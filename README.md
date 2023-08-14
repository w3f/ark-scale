# Arkworks serialization wrapped in Parity SCALE codec

`ArkScale(T)` can be serialized or deserialized using parity-scale-codec,
provided `T` can be serialized or deserialized using ark-serialize.

Arkworks serializes via the `std::io::{Read,Write}` traits, or its
no_std fork of those traits, as do other zcash sapling derivatives.
At its core, Parity SCALE codec also consists of traits `{Input,Output}`
analogous to `std::io::{Read,Write}` respectively, as well as traits
`{Decode,Encode}` also quite similar to
 `ark-serialize::{CanonicalDeserialize,CanonicalSerialize}`.
We simply translate between these extremely similar traits, including
wrapping and unwrapping errors appropriately.

`ArkScale` cannot easily implement `MaxEncodedLen` or `ConstEncodedLen`
from SCALE, due to the orphan rules.  You'll need these if using weights
in Frame, so you should usually create wrapper types around `ArkScale`.
As a rule, anytime you choose curves then you'll have a conditionanl
ark-substrate dependence anyways, so wrapper types should not become
too onerous, and they likely improve documentation, errors, etc anyways.

`ArkScale` panics if serialization fails because SCALE does not propogate
serialization failures.  `ArkScale` users should therefore be responcible
for ensuring `T: CanonicalSerialize` cannot fail for mathematical reasons,
at least when using `ArkScale` in Polkadot.  In principle, any serialization
code could fail from memory exaustion, but Frame avoids this, provided
you've set `MaxEncodedLen` correctly on your wrapper types.

