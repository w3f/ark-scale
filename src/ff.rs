
use crate::{ArkScaleMaxEncodedLen}; // ArkScale,ArkScaleRef,ConstEncodedLen

use ark_ff::fields::models::*;


impl<P: FpConfig<N>, const N: usize> ArkScaleMaxEncodedLen for Fp<P,N> {
    crate::impl_body_max_encode_len!();
}

impl<C: QuadExtConfig> ArkScaleMaxEncodedLen for QuadExtField<C> {
    crate::impl_body_max_encode_len!();
}

impl<C: CubicExtConfig> ArkScaleMaxEncodedLen for CubicExtField<C> {
    crate::impl_body_max_encode_len!();
}

