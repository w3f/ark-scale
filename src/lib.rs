// -*- mode: rust; -*-
//
// Copyright (c) 2019 Web 3 Foundation
//
// Authors:
// - Jeffrey Burdges <jeff@web3.foundation>

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(unsafe_code)]
#![doc = include_str!("../README.md")]

use ark_std::{
    borrow::Borrow,
    fmt,
    io::{self, Read, Write},
    vec::Vec,
};

type ArkResult<T> = Result<T, io::Error>;
use ark_serialize::{
    CanonicalDeserialize, CanonicalSerialize, Compress, SerializationError, Validate,
};
pub use ark_serialize::{self as ark_serialize};

pub use scale_codec::{self as scale, MaxEncodedLen}; // max_encoded_len::ConstEncodedLen
use scale::{Decode, Encode, EncodeLike, Input, Output};
use scale_info::TypeInfo;
// type ScaleResult<T> = Result<T,scale::Error>;

pub mod rw;
use rw::*;

mod max_encoded_len;
pub use max_encoded_len::*;

#[cfg(feature = "hazmat")]
pub mod hazmat;

#[cfg(feature = "ff")]
pub mod ff;

#[cfg(feature = "ec")]
pub mod ec;

#[cfg(test)]
mod tests;

/*
error: `(Compress, Validate)` is forbidden as the type of a const generic parameter
   --> src/lib.rs:145:33
    = note: the only supported types are integers, `bool` and `char`
*/

/// Arkworks' serialization modes, morally (Compress, Validate) but
/// const generics only supports integers, `bool` and `char` right now.
pub type Usage = u8; // (Compress, Validate)

/// Arkworks' serialization modes hack.
pub const fn make_usage(compress: Compress, validate: Validate) -> Usage {
    let c = match compress {
        Compress::Yes => 0,
        Compress::No => 1,
    };
    let v = match validate {
        Validate::Yes => 0,
        Validate::No => 2,
    };
    c | v
}

pub const fn is_compressed(u: Usage) -> Compress {
    // u.0
    assert!(u < 4);
    if u & 1 == 1 {
        Compress::No
    } else {
        Compress::Yes
    }
}

pub const fn is_validated(u: Usage) -> Validate {
    // u.1
    assert!(u < 4);
    if u & 2 == 2 {
        Validate::No
    } else {
        Validate::Yes
    }
}

/// ArkScale usage for typical wire formats, like block data and gossip messages.  Always safe.
pub const WIRE: Usage = make_usage(Compress::Yes, Validate::Yes);

/// ArkScale usage which neither compresses nor validates inputs,
/// only for usage in host calls and on-chain storage where the runtime already performed
/// validation checks.
pub const HOST_CALL: Usage = make_usage(Compress::No, Validate::No);

/// Arkworks type wrapped for serialization by Scale
#[derive(Clone, Eq, PartialEq, Debug)] // CanonicalSerialize, CanonicalDeserialize
#[repr(transparent)]
pub struct ArkScale<T, const U: Usage = WIRE>(pub T);

impl<T, const U: Usage> From<T> for ArkScale<T, U> {
    fn from(t: T) -> ArkScale<T, U> {
        ArkScale(t)
    }
}

impl<T: CanonicalDeserialize, const U: Usage> Decode for ArkScale<T, U> {
    fn decode<I: Input>(input: &mut I) -> Result<Self, scale::Error> {
        <T as CanonicalDeserialize>::deserialize_with_mode(
            InputAsRead(input),
            is_compressed(U),
            is_validated(U),
        )
        .map(|v| ArkScale(v))
        .map_err(ark_error_to_scale_error)
    }

    // fn skip<I: Input>(input: &mut I) -> Result<(), Error> { ... }

    // fn encoded_fixed_size() -> Option<usize> { ... }
}

const OOPS: &'static str =
    "Arkworks serialization failed, but Scale cannot handle serialization failures.  As ark_scale::rw::OutputAsWrite cannot fail, and ark_serialize_derive cannot introduce fresh falures, you have a non-derived `impl<..> ark_serialize::CanonicalSerialize` which fails, which violates usage conditions from ark-scale/README.md.";
    // You could usually verify this condition by reading results like
    // git clone https://github.com/arkworks-rs/algebra
    // cd algebra
    // grep -r --include '*.rs' 'CanonicalSerialize for' -A 10 ff* ec* poly/ | less


impl<T: CanonicalSerialize, const U: Usage> EncodeLike for ArkScale<T, U> {}

impl<T: CanonicalSerialize, const U: Usage> Encode for ArkScale<T, U> {
    fn size_hint(&self) -> usize {
        self.0.serialized_size(is_compressed(U))
    }

    fn encode_to<O: Output + ?Sized>(&self, dest: &mut O) {
        self.0
            .serialize_with_mode(OutputAsWrite(dest), is_compressed(U))
            .expect(OOPS);
    }

    // TODO:  Arkworks wants an io::Write, so we ignre the rule that
    // value types override using_encoded.
    // fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R;

    fn encoded_size(&self) -> usize {
        self.0.serialized_size(is_compressed(U))
    }
}

impl<T: 'static + ArkScaleMaxEncodedLen, const U: Usage> TypeInfo for ArkScale<T, U> {
    type Identity = Self;

    fn type_info() -> scale_info::Type {
        let path = scale_info::Path::new("ArkScale", module_path!());
        let array_type_def = scale_info::TypeDefArray {
            len: T::max_encoded_len(is_compressed(U)) as u32,
            type_param: scale_info::MetaType::new::<u8>(),
        };
        let type_def = scale_info::TypeDef::Array(array_type_def);
        scale_info::Type { path, type_params: Vec::new(), type_def, docs: Vec::new() }
    }
}


#[derive(Copy,Debug)] // CanonicalSerialize
pub struct ArkScaleRef<'a, T, const U: Usage = WIRE>(pub &'a T);

impl<'a, T, const U: Usage> Clone for ArkScaleRef<'a, T, U> {
    fn clone(&self) -> Self {
        ArkScaleRef(self.0)
    }
}

impl<'a, T, const U: Usage> From<&'a T> for ArkScaleRef<'a, T, U> {
    fn from(t: &'a T) -> ArkScaleRef<'a, T, U> {
        ArkScaleRef(t)
    }
}

impl<'a, T: CanonicalSerialize, const U: Usage> Encode for ArkScaleRef<'a, T, U> {
    fn size_hint(&self) -> usize {
        self.0.serialized_size(is_compressed(U))
    }

    fn encode_to<O: Output + ?Sized>(&self, dest: &mut O) {
        self.0
            .serialize_with_mode(OutputAsWrite(dest), is_compressed(U))
            .expect(OOPS);
    }

    // TODO:  Arkworks wants an io::Write, so we ignre the rule that
    // value types override using_encoded.
    // fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R;

    fn encoded_size(&self) -> usize {
        self.0.serialized_size(is_compressed(U))
    }
}


/// Arkworks' `CanonicalSerialize` cannot consume `Iterator`s directly,
/// but `iter_ark_to_ark_bytes` serializes exactly like `Vec<T>`,
/// `&'a [T]`, or `[T]` do with `CanonicalSerialize`.
///
/// Returns errors as `ark_serialize::SerializationError`.
pub fn iter_ark_to_ark_bytes<T, B, I>(iter: I, usage: Usage) -> Result<Vec<u8>, SerializationError>
where
    T: CanonicalSerialize,
    B: Borrow<T>,
    I: IntoIterator<Item = B>,
{
    const LL: usize = 8;
    let mut iter = iter.into_iter();
    let len = iter.size_hint().0;
    let first = iter.next();
    let mut vec = if let Some(ref e) = first {
        let size = e.borrow().serialized_size(is_compressed(usage));
        Vec::with_capacity(LL + size * (1 + len))
    } else {
        Vec::with_capacity(LL)
    };
    vec.extend_from_slice(&[0u8; LL]);
    if let Some(e) = first {
        e.borrow()
            .serialize_with_mode(&mut vec, is_compressed(usage))?;
        let mut l = 1;
        for e in iter {
            e.borrow()
                .serialize_with_mode(&mut vec, is_compressed(usage))?;
            l += 1;
        }
        debug_assert_eq!(
            l, len,
            "Iterator::size_hint underestimate would slow down release execution."
        );
        // let err = |_| scale_error_to_ark_error(scale::Error::from("Arkworks cannot serialize more than 2^32 items."));
        // let l = u32::try_from(l).map_err(err) ?;
        (&mut vec)[0..LL].copy_from_slice(&(l as u64).to_le_bytes());
    }
    Ok(vec)
}

/// Arkworks' `CanonicalSerialize` cannot consume `Iterator`s directly,
/// but `iter_ark_to_scale_bytes` serializes exactly like
/// `ArkScale(Vec<T>)`, `ArkScale(&'a [T])`, or `ArkScale([T])` do
/// under `parity_scale_codec::Encode`.
///
/// Returns errors as `parity_scale_codec::Error`.
pub fn iter_ark_to_scale_bytes<T, B, I>(iter: I, usage: Usage) -> Result<Vec<u8>, scale::Error>
where
    T: CanonicalSerialize,
    B: Borrow<T>,
    I: IntoIterator<Item = B>,
{
    iter_ark_to_ark_bytes(iter, usage).map_err(ark_error_to_scale_error)
}


// We next provide helper macros for implementing scale upon
// your own arkworks types.


/// Implement body of `scale::Decode` by delegation to `ArkScale`,
/// usable from polymorphic code.
#[macro_export]
macro_rules! impl_decode_via_ark {
    () => {
        fn decode<I: ark_scale::scale::Input>(input: &mut I) -> Result<Self, ark_scale::scale::Error> {
            let a: ark_scale::ArkScale<Self> = <ark_scale::ArkScale<Self> as ark_scale::scale::Decode>::decode(input) ?;
            Ok(a.0)
        }
    
        /*
        fn decode_into<I: ark_scale::scale::Input>(input: &mut I, dst: &mut core::mem::MaybeUninit<Self>) -> Result<ark_scale::scale::DecodeFinished, ark_scale::scale::Error> {
            // safe thanks to #[repr(transparent)]
            <ark_scale::ArkScale<Self> as ark_scale::scale::Decode>::decode_into(input,dst)
        }
        */
    
        fn skip<I: ark_scale::scale::Input>(input: &mut I) -> Result<(), ark_scale::scale::Error> {
            <ark_scale::ArkScale<Self> as ark_scale::scale::Decode>::skip(input)
        }
    
        fn encoded_fixed_size() -> Option<usize> {
            <ark_scale::ArkScale<Self> as ark_scale::scale::Decode>::encoded_fixed_size()
        }    
    }
}

/// Implement body of `scale::Encode` by delegation to `ArkScale`,
/// usable from polymorphic code.
#[macro_export]
macro_rules! impl_encode_via_ark {
    () => {
        fn size_hint(&self) -> usize {
            let a: ark_scale::ArkScaleRef<Self> = ark_scale::ArkScaleRef(self);
            a.size_hint()
        }
    
        fn encode_to<O: ark_scale::scale::Output + ?Sized>(&self, dest: &mut O) {
            let a: ark_scale::ArkScaleRef<Self> = ark_scale::ArkScaleRef(self);
            a.encode_to(dest)
        }
    
        fn encode(&self) -> Vec<u8> {
            let a: ark_scale::ArkScaleRef<Self> = ark_scale::ArkScaleRef(self);
            a.encode()
        }
    
        fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R {
            let a: ark_scale::ArkScaleRef<Self> = ark_scale::ArkScaleRef(self);
            a.using_encoded(f)
        }
    
        fn encoded_size(&self) -> usize {
            let a: ark_scale::ArkScaleRef<Self> = ark_scale::ArkScaleRef(self);
            a.encoded_size()
        }
    }
}

/// Implement `scale::{Encode,Decode}` by delegation to `ArkScale`,
/// but lacks support for polymorphic code.
/// 
/// You should manually provide `MaxEncodedLen` for weights.
/// ```ignore
/// impl_scale_via_ark!(MyArkworksType);
/// 
/// impl ark_scale::MaxEncodedLen for MyArkworksType 
/// {
///     fn max_encoded_len() -> usize {
///         256
///     }
/// }
/// ```
#[macro_export]
macro_rules! impl_scale_via_ark {
    ($t:ty) => {
        impl Decode for $t {
            ark_scale::impl_decode_via_ark!();
        }

        impl Encode for $t {
            ark_scale::impl_encode_via_ark!();
        }

        impl ark_scale::scale::EncodeLike for $t {}
    }
} // macro_rules! impl_scale_via_ark

/// Implement body of `scale::MaxEncodedLen` by delegation
/// to `ark_std::Zero` and `CanonicalSerialize`, usable
/// from polymorphic code.
#[macro_export]
macro_rules! impl_body_max_encode_len {
    () => {
        ark_scale::impl_body_max_encode_len!(Self);
    };
    ($t:ty) => {
        #[inline]
        fn max_encoded_len(compress: ark_serialize::Compress) -> usize {
            use ark_serialize::{CanonicalSerialize}; 
            <$t as ark_std::Zero>::zero().serialized_size(compress)
        }
    };
}
