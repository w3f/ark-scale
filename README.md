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
serialization failures.  As scale outputs cannot fail, and ark-scale-derive
does not introduce failures, we therefore cannot trigger this panic except
by some explicit `impl CanonicalSerialize for T` intrpducing a failure.
`ArkScale` users should therefore be responcible for reviewing non-derived
`CanonicalSerialize` in their dependencies.  In particular, there are no
fresh failures in arkworks/algebra:
```bash
git clone https://github.com/arkworks-rs/algebra
cd algebra
grep -r --include '*.rs' 'CanonicalSerialize for' -A 10 ff* ec* poly/ | less
```

