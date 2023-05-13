//! This crate provides unaligned integer typed of standard and non-standard bitwidths.
//!
//! These types mainly allow for conversion from and to Rust primitive integer types
//! and do not support arithmetic operations on themselves directly. Instead convert
//! to the next best Rust primitive type in order to perform arithmetic operations.
//!
//! The main advantage of these types is that their alignment is always 1 which allows
//! to have more control over data layout in Rust `enum` and `struct` types that are
//! making use of these types.
//! Note that generally unaligned values will probably slow down fetching and storing
//! of values so do not use these types without profiling for your specific use case.
//!
//! # Example
//!
//! ```
//! pub enum Aligned {
//!     A(u64),
//!     B(i64),
//! }
//!
//! assert_eq!(core::mem::size_of::<Aligned>(), 16);
//! ```
//!
//! ```
//! # use unaligned_int::{U16, I16, U32, I32, U64, I64};
//! pub enum Unaligned {
//!     A(U64),
//!     B(I64),
//! }
//!
//! // Note that the size of `Op` is 4 bytes since `U24` has an alignemnt of 1.
//! assert_eq!(core::mem::size_of::<Unaligned>(), 9);
//! ```

#![no_std]

mod error;
mod from;
mod signed;
mod standard;
mod try_from;
mod unsigned;
mod utils;
mod within_bounds;

#[cfg(test)]
mod tests;

pub use self::error::TryFromIntError;
pub use self::signed::{I104, I112, I120, I24, I40, I48, I56, I72, I80, I88, I96};
pub use self::standard::{I128, I16, I32, I64, U128, U16, U32, U64};
pub use self::unsigned::{U104, U112, U120, U24, U40, U48, U56, U72, U80, U88, U96};
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
