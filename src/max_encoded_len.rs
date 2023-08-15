
use crate::{ArkScale,ArkScaleRef,WIRE,MaxEncodedLen}; // ConstEncodedLen

use ark_serialize::{CanonicalSerialize};  // CanonicalDeserialize, 


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

impl<T> MaxEncodedLen for ArkScale<T, WIRE> 
where T: CanonicalSerialize+ArkScaleMaxEncodedLen,
{
    fn max_encoded_len() -> usize {
        <T as ArkScaleMaxEncodedLen>::max_encoded_len()
    }
}

impl<'a,T> MaxEncodedLen for ArkScaleRef<'a,T, WIRE> 
where T: CanonicalSerialize+ArkScaleMaxEncodedLen,
{
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
