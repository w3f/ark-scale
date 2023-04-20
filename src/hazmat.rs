//! # Hazardous projective serialization and deserialization
//!
//! Do NOT use this module in wire formats or other untrusted cases.
//!
//! Aside from not being cannonical, we caution that projective coordinates
//! actually [leak secret key material](https://eprint.iacr.org/2003/191),
//! which makes them extraordinarily dangerous.
//!
//! We do not implement curve or subgroup check for elliptic curve points
//! in projective coordinates, nor do we validate their base field elements.
//! You diserve to be p0wned if you use them in an untrusted enviroment.

use super::*;
pub use ark_ec::models::{short_weierstrass as sw, twisted_edwards as te};

pub struct ArkScaleProjective<T>(pub T);

impl<T> From<T> for ArkScaleProjective<T> {
    fn from(t: T) -> ArkScaleProjective<T> {
        ArkScaleProjective(t)
    }
}

/// Uncompressed mode since projective coordinates are non-cannonical and leaks secrets
const MC: Compress = Compress::No;
const MV: Validate = Validate::No;

// Short Weierstrass //

pub fn ark_sw_encode_to<W, C>(p: &sw::Projective<C>, dest: &mut W) -> Result<(), SerializationError>
where
    W: Write + ?Sized,
    C: sw::SWCurveConfig,
{
    p.x.serialize_with_mode(&mut *dest, MC)?;
    p.y.serialize_with_mode(&mut *dest, MC)?;
    p.z.serialize_with_mode(&mut *dest, MC)
}

pub fn scale_sw_encode_to<O, C>(p: &sw::Projective<C>, dest: &mut O)
where
    O: Output + ?Sized,
    C: sw::SWCurveConfig,
{
    ark_sw_encode_to(p, &mut OutputAsWrite(dest)).expect(OOPS);
}

impl<C: sw::SWCurveConfig> Encode for ArkScaleProjective<sw::Projective<C>> {
    fn size_hint(&self) -> usize {
        3 * self.0.x.serialized_size(MC)
    }

    fn encode_to<O: Output + ?Sized>(&self, dest: &mut O) {
        scale_sw_encode_to(&self.0.borrow(), dest)
    }

    fn encoded_size(&self) -> usize {
        3 * self.0.serialized_size(MC)
    }
}

impl<'a, C: sw::SWCurveConfig> Encode for ArkScaleProjective<&'a sw::Projective<C>> {
    fn size_hint(&self) -> usize {
        3 * self.0.x.serialized_size(MC)
    }

    fn encode_to<O: Output + ?Sized>(&self, dest: &mut O) {
        scale_sw_encode_to(&self.0.borrow(), dest)
    }

    fn encoded_size(&self) -> usize {
        3 * self.0.serialized_size(MC)
    }
}

pub fn ark_sw_decode_from<R, C>(src: &mut R) -> Result<sw::Projective<C>, SerializationError>
where
    R: Read,
    C: sw::SWCurveConfig,
{
    let x = C::BaseField::deserialize_with_mode(&mut *src, MC, MV)?;
    let y = C::BaseField::deserialize_with_mode(&mut *src, MC, MV)?;
    let z = C::BaseField::deserialize_with_mode(&mut *src, MC, MV)?;
    Ok(sw::Projective { x, y, z })
}

pub fn scale_sw_decode_from<I, C>(src: &mut I) -> Result<sw::Projective<C>, scale::Error>
where
    I: Input,
    C: sw::SWCurveConfig,
{
    ark_sw_decode_from(&mut InputAsRead(src)).map_err(ark_error_to_scale_error)
}

impl<C: sw::SWCurveConfig> Decode for ArkScaleProjective<sw::Projective<C>> {
    fn decode<I: Input>(input: &mut I) -> Result<Self, scale::Error> {
        scale_sw_decode_from(input).map(|p| ArkScaleProjective(p))
    }

    // fn skip<I: Input>(input: &mut I) -> Result<(), Error> { ... }

    // fn encoded_fixed_size() -> Option<usize> { ... }
}

// Twisted Edwards //

pub fn ark_te_encode_to<W, C>(p: &te::Projective<C>, dest: &mut W) -> Result<(), SerializationError>
where
    W: Write + ?Sized,
    C: te::TECurveConfig,
{
    p.x.serialize_with_mode(&mut *dest, MC)?;
    p.y.serialize_with_mode(&mut *dest, MC)?;
    p.t.serialize_with_mode(&mut *dest, MC)?;
    p.z.serialize_with_mode(&mut *dest, MC)
}

pub fn scale_te_encode_to<O, C>(p: &te::Projective<C>, dest: &mut O)
where
    O: Output + ?Sized,
    C: te::TECurveConfig,
{
    ark_te_encode_to(p, &mut OutputAsWrite(dest)).expect(OOPS);
}

impl<C: te::TECurveConfig> Encode for ArkScaleProjective<te::Projective<C>> {
    fn size_hint(&self) -> usize {
        4 * self.0.x.serialized_size(MC)
    }

    fn encode_to<O: Output + ?Sized>(&self, dest: &mut O) {
        scale_te_encode_to(&self.0, dest)
    }

    fn encoded_size(&self) -> usize {
        4 * self.0.serialized_size(MC)
    }
}

impl<'a, C: te::TECurveConfig> Encode for ArkScaleProjective<&'a te::Projective<C>> {
    fn size_hint(&self) -> usize {
        4 * self.0.x.serialized_size(MC)
    }

    fn encode_to<O: Output + ?Sized>(&self, dest: &mut O) {
        scale_te_encode_to(&self.0, dest)
    }

    fn encoded_size(&self) -> usize {
        4 * self.0.serialized_size(MC)
    }
}

pub fn ark_te_decode_from<R, C>(src: &mut R) -> Result<te::Projective<C>, SerializationError>
where
    R: Read,
    C: te::TECurveConfig,
{
    let x = C::BaseField::deserialize_with_mode(&mut *src, MC, MV)?;
    let y = C::BaseField::deserialize_with_mode(&mut *src, MC, MV)?;
    let t = C::BaseField::deserialize_with_mode(&mut *src, MC, MV)?;
    let z = C::BaseField::deserialize_with_mode(&mut *src, MC, MV)?;
    Ok(te::Projective { x, y, z, t })
}

pub fn scale_te_decode_from<I, C>(src: &mut I) -> Result<te::Projective<C>, scale::Error>
where
    I: Input,
    C: te::TECurveConfig,
{
    ark_te_decode_from(&mut InputAsRead(src)).map_err(ark_error_to_scale_error)
}

impl<C: te::TECurveConfig> Decode for ArkScaleProjective<te::Projective<C>> {
    fn decode<I: Input>(input: &mut I) -> Result<Self, scale::Error> {
        scale_te_decode_from(input).map(|p| ArkScaleProjective(p))
    }

    // fn skip<I: Input>(input: &mut I) -> Result<(), Error> { ... }

    // fn encoded_fixed_size() -> Option<usize> { ... }
}
