
use ark_serialize::Compress;

use crate::ArkScaleMaxEncodedLen; // ArkScale,ArkScaleRef,ConstEncodedLen

use ark_ec::models::{short_weierstrass as sw, twisted_edwards as te};

impl<P: sw::SWCurveConfig> ArkScaleMaxEncodedLen for sw::Affine<P> {
    #[inline]
    fn max_encoded_len(compress: Compress) -> usize {
        P::serialized_size(compress)
    }
}

impl<P: te::TECurveConfig> ArkScaleMaxEncodedLen for te::Affine<P> {
    #[inline]
    fn max_encoded_len(compress: Compress) -> usize {
        P::serialized_size(compress)
    }
}
