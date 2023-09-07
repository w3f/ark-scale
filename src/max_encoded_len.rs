
use crate::{
    ArkScale,ArkScaleRef,WIRE,
    MaxEncodedLen,
    scale::{self, Decode, Encode, EncodeLike, Input, Output}, // ConstEncodedLen
};

use ark_serialize::{CanonicalSerialize,CanonicalDeserialize,Compress,Validate};


/// An orphan rules helper which provides
/// `impl scale::MaxEncodedLen for ArkScale<T,WIRE>`
/// 
/// We suggest wrapper types in preference to this trait becuase
/// on-chain verifier code becomes substrate aware anyways whenever
/// you choose curves, via a conditional ark-substrate dependence. 
/// Also wrappers can improve documentation and errors.
pub trait ArkScaleMaxEncodedLen {
    /// Upper bound, in bytes, of the maximum encoded size of this item.
   fn max_encoded_len() -> usize;
}

impl ArkScaleMaxEncodedLen for () {
    #[inline]
    fn max_encoded_len() -> usize { 0 }
}

impl<T> MaxEncodedLen for ArkScale<T, WIRE> 
where T: CanonicalSerialize+ArkScaleMaxEncodedLen,
{
    #[inline]
    fn max_encoded_len() -> usize {
        <T as ArkScaleMaxEncodedLen>::max_encoded_len()
    }
}

impl<'a,T> MaxEncodedLen for ArkScaleRef<'a,T, WIRE> 
where T: CanonicalSerialize+ArkScaleMaxEncodedLen,
{
    #[inline]
    fn max_encoded_len() -> usize {
        <T as ArkScaleMaxEncodedLen>::max_encoded_len()
    }
}

/* 
/// An orphan rules helper which provides
/// `impl scale::ConstEncodedLen for ArkScale<T,WIRE>`
pub trait ArkScaleConstEncodedLen: ArkScaleMaxEncodedLen {}
 
impl<T> ConstEncodedLen for ArkScale<T, WIRE> 
where T: CanonicalSerialize+ArkScaleConstEncodedLen,
{ }

impl<'a,T> ConstEncodedLen for ArkScaleRef<'a, T, WIRE> 
where T: CanonicalSerialize+ArkScaleConstEncodedLen,
{ }

*/


/// Arkworks type wrapped for serialization by Scale
#[derive(Clone, Eq, PartialEq, Debug)] // CanonicalSerialize, CanonicalDeserialize
#[repr(transparent)]
pub struct ArkScaleLen<T, const L: usize>(pub T);

impl<T, const L: usize> From<T> for ArkScaleLen<T,L> {
    fn from(t: T) -> ArkScaleLen<T,L> {
        ArkScaleLen(t)
    }
}

impl<T: CanonicalDeserialize, const L: usize> Decode for ArkScaleLen<T, L> {
    fn decode<I: Input>(input: &mut I) -> Result<Self, scale::Error> {
        <T as CanonicalDeserialize>::deserialize_with_mode(
            crate::InputAsRead(input),
            Compress::Yes,
            Validate::Yes,
        )
        .map(|v| ArkScaleLen(v))
        .map_err(crate::ark_error_to_scale_error)
    }

    // fn skip<I: Input>(input: &mut I) -> Result<(), Error> { ... }

    // fn encoded_fixed_size() -> Option<usize> { ... }
}

impl<T: CanonicalSerialize, const L: usize> EncodeLike for ArkScaleLen<T, L> {}

impl<T: CanonicalSerialize, const L: usize> Encode for ArkScaleLen<T, L> {
    fn size_hint(&self) -> usize {
        self.encoded_size()
    }

    fn encode_to<O: Output + ?Sized>(&self, dest: &mut O) {
        self.0
            .serialize_with_mode(crate::OutputAsWrite(dest), Compress::Yes)
            .expect(crate::OOPS);
    }

    // TODO:  Arkworks wants an io::Write, so we ignre the rule that
    // value types override using_encoded.
    // fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R;

    fn encoded_size(&self) -> usize {
        let l = self.0.serialized_size(Compress::Yes);
        debug_assert!(l <= L, "ArkScaleLen has inforrect length specified.");
        l
    }
}

impl<T: CanonicalDeserialize, const L: usize> ArkScaleMaxEncodedLen for ArkScaleLen<T, L> {
    /// Upper bound, in bytes, of the maximum encoded size of this item.
   fn max_encoded_len() -> usize { L }
}
