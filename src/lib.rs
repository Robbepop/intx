//! This crate provides new integer types with non-standard and fixed bitwidths
//! such as `U24`, `I48`, `U96` and so forth with a focus on data layout and alignment.
//!
//! - All integers provided by this crate require the minimum number of bytes for their representation.
//!   For example, `U24` requires 3 bytes, `I48` requires 6 bytes.
//! - The alignment of all integer types provided by this crate is always 1. If another
//!   alignment is required it is recommended to wrap the integer type in a newtype and
//!   enforce an alignment via `#[align(N)]`.
//! - As of now the provided integers do not have a rich set of arithmetic methods defined on them.
//!   It is instead expected to convert them to Rust primitive integers, apply the computation and
//!   eventually convert the result back. This might be supported in the future if requested.
//! - The binary representation of integer types provided by this crate is in twos-complement just
//!   like Rust's built-in integer types.
//!
//! ## Data Layout
//!
//! All integer types provided by this crate internally consists of a single byte array.
//! For example, the structure of `U24` is `struct U24([u8; 3]);` allowing for optimal memory
//! usage and an alignment of 1 (if needed).
//!
//! ## API
//!
//! Integer types provided by this crate only have very a minimal API surface.
//!
//! - Traits implemented by all of the integer types are the following:
//!
//!   - `Clone`, `Copy`, `Default`, `Eq`, `PartialEq`, `Ord`, `PartialOrd`, `Hash`
//!     - Common traits are all implemented as efficiently as possible for every integer type.
//!   - `Debug`, `Display`, `Binary`, `Octal`, `LowerHex`, `UpperHex`, `LowerExp`, `UpperExp`
//!     - Integer types mimick the display representation of the next larger Rust built-in integer type.
//!
//! - Endian-aware conversion routines are also implemented:
//!
//!   - `from_ne_bytes`, `to_ne_bytes`: Convert from and to native-endian bytes. (always efficient)
//!   - `from_le_bytes`, `to_le_bytes`: Convert from and to little-endian bytes.
//!   - `from_be_bytes`, `to_be_bytes`: Convert from and to big-endian bytes.
//!
//! - Rich `From` and `TryFrom` implementations:
//!
//!   - All provided integer types have a very rich set of `From` and `TryFrom` trait implementations
//!     to efficiently convert between different integer types and Rust built-in integers.
//!
//!
//! # Example: Packed
//!
//! Here the Rust compiler wastes 3 bytes for the discriminent of the `enum`.
//! ```
//! pub enum Unpacked {
//!     A(u32), // We only use the lowest 24-bit of the integer.
//!     B(i16),
//! }
//! assert_eq!(core::mem::size_of::<Unpacked>(), 8);
//! ```
//!
//! Using `intx::U24` the Rust compiler can properly pack the `enum` type wasting no bytes.
//! ```
//! pub enum Packed {
//!     A(intx::U24), // Now using a type that reflects our usage intent properly.
//!     B(i16),
//! }
//! assert_eq!(core::mem::size_of::<Packed>(), 4);
//! ```
//!
//! # Example: Alignment
//!
//! With standard alignment the `enum` discriminent takes up a whopping 8 bytes.
//! ```
//! pub enum Aligned {
//!     A(u64),
//!     B(i64),
//! }
//! assert_eq!(core::mem::size_of::<Aligned>(), 16);
//! ```
//!
//! Using `intx` integers with their alignment of 1 allows to pack the `enum` discrimant to a single byte.
//! ```
//! pub enum Unaligned {
//!     A(intx::U64),
//!     B(intx::I64),
//! }
//! assert_eq!(core::mem::size_of::<Unaligned>(), 9);
//! ```

#![no_std]

mod defs;
mod error;
mod from;
mod try_from;
mod utils;
mod within_bounds;

#[cfg(test)]
mod tests;

pub use self::defs::{
    I104, I112, I120, I128, I16, I24, I32, I40, I48, I56, I64, I72, I80, I88, I96, U104, U112,
    U120, U128, U16, U24, U32, U40, U48, U56, U64, U72, U80, U88, U96,
};
pub use self::error::TryFromIntError;
pub(crate) use self::within_bounds::IsWithinBoundsOf;

/// Trait implemented by Rust integer primitives to communicate their bounds.
trait BoundedInteger: Sized {
    /// The minimum value representable by `Self`.
    const MIN: Self;
    /// The maximum value representable by `Self`.
    const MAX: Self;
}
macro_rules! impl_bounded_integer_for {
    ( $( $prim:ty ),* $(,)? ) => {
        $(
            impl BoundedInteger for $prim {
                const MIN: Self = <$prim>::MIN;
                const MAX: Self = <$prim>::MAX;
            }
        )*
    };
}
impl_bounded_integer_for!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128);

/// Trait implemented by unaligned integers provided by this crate.
trait UnalignedInteger: Sized {
    /// The smallest integer primitive type that is larger than `Self`.
    ///
    /// # Example
    ///
    /// For `U24` this is `u32`.
    type Repr: BoundedInteger + TryInto<Self> + From<Self>;

    /// Returns the sign extension byte for the unaligned integer value.
    ///
    /// # Note
    ///
    /// Basically this returns `0x00` for positive or unsigned integer
    /// values and `0xFF` for signed negative integer values.
    fn sign_ext_byte(self) -> u8;
}

macro_rules! impl_unaligned_uint_for {
    ( $( $ty:ty ),* ) => {
        $(
            impl $crate::UnalignedInteger for $ty {
                type Repr = Self;

                #[inline]
                fn sign_ext_byte(self) -> u8 {
                    0x00_u8
                }
            }
        )*
    };
}
impl_unaligned_uint_for!(u8, u16, u32, u64, u128);

macro_rules! impl_unaligned_int_for {
    ( $( $ty:ty ),* ) => {
        $(
            impl $crate::UnalignedInteger for $ty {
                type Repr = Self;

                #[inline]
                fn sign_ext_byte(self) -> u8 {
                    $crate::utils::sign_ext_byte(self.is_positive())
                }
            }
        )*
    };
}
impl_unaligned_int_for!(i8, i16, i32, i64, i128);
