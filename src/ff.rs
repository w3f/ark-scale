
use ark_std::Zero;
use ark_serialize::{CanonicalSerialize};

use crate::{ArkScaleMaxEncodedLen}; // ArkScale,ArkScaleRef,ConstEncodedLen

use ark_ff::fields::models::*;


impl<P: FpConfig<N>, const N: usize> ArkScaleMaxEncodedLen for Fp<P,N> {
    #[inline]
    fn max_encoded_len() -> usize {
        Self::zero().compressed_size()
    }
}

macro_rules! max_encode_len_from_zero {
    ($f:ident,$c:ident) => {

impl<C: $c> ArkScaleMaxEncodedLen for $f<C> {
    #[inline]
    fn max_encoded_len() -> usize {
        Self::zero().compressed_size()
    }
}

    }
} // macro_rules! max_encode_len_from_zero

max_encode_len_from_zero!(CubicExtField,CubicExtConfig);
max_encode_len_from_zero!(QuadExtField,QuadExtConfig);
