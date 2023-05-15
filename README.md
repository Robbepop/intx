# `intx` - Non-standard fixed bitwidth integers for Rust

This crate provides new integer types with non-standard and fixed bitwidths
such as `U24`, `I48`, `U96` and so forth with a focus on data layout and alignment.

- All integers provided by this crate require the minimum number of bytes for their representation.
  For example, `U24` requires 3 bytes, `I48` requires 6 bytes.
- The alignment of all integer types provided by this crate is always 1. If another
  alignment is required it is recommended to wrap the integer type in a newtype and
  enforce an alignment via `#[align(N)]`.
- As of now the provided integers do not have a rich set of arithmetic methods defined on them.
  It is instead expected to convert them to Rust primitive integers, apply the computation and
  eventually convert the result back. This might be supported in the future if requested.
- The binary representation of integer types provided by this crate is in twos-complement just
  like Rust's built-in integer types.

## Data Layout

All integer types provided by this crate internally consists of a single byte array.
For example, the structure of `U24` is `struct U24([u8; 3]);` allowing for optimal memory
usage and an alignment of 1 (if needed).

## API

Integer types provided by this crate only have very a minimal API surface.

- Traits implemented by all of the integer types are the following:

  - `Clone`, `Copy`, `Default`, `Eq`, `PartialEq`, `Ord`, `PartialOrd`, `Hash`
    - Common traits are all implemented as efficiently as possible for every integer type.
  - `Debug`, `Display`, `Binary`, `Octal`, `LowerHex`, `UpperHex`, `LowerExp`, `UpperExp`
    - Integer types mimick the display representation of the next larger Rust built-in integer type.

- Endian-aware conversion routines are also implemented:

  - `from_ne_bytes`, `to_ne_bytes`: Convert from and to native-endian bytes. (always efficient)
  - `from_le_bytes`, `to_le_bytes`: Convert from and to little-endian bytes.
  - `from_be_bytes`, `to_be_bytes`: Convert from and to big-endian bytes.

- Rich `From` and `TryFrom` implementations:

  - All provided integer types have a very rich set of `From` and `TryFrom` trait implementations
    to efficiently convert between different integer types and Rust built-in integers.

## Usage

The focus of this crate is data layout, alignment and space-optization.
It was crafted when the `wasmi` interpreter required a more memory efficient representation
of its internal bytecode. Using a `U24` instead of `u32` in certain places allows the Rust
compiler to pack data structures more efficiently in some cases, especially when using `enum` types.

If your primary focus is on the logical side where you are mostly interested in
arithmetic operations on integer types with non-standard but fixed bitwidths then
maybe the [`ux` crate](https://crates.io/crates/ux) is a better fit for you.

## Alternatives

The alternative [`ux` crate](https://crates.io/crates/ux) provides non-standard
and fixed bitwidth integers as well but the focus of both crates is very different.

| Property | `intx` | `ux` |
|---|---|---|
| [`size_of`](https://doc.rust-lang.org/core/mem/fn.size_of.html) | All integer types require the minimum number of bytes for their representation. For example, `size_of<intx::U24>() == 3` | All integer types have the same `size_of` as the next biggest Rust built-in integer primitive. For example, `size_of<ux::u24>() == size_of<u32>() == 4` |
| [`align_of`](https://doc.rust-lang.org/core/mem/fn.align_of.html) | All integer types have an alignment of 1. If another alignment is needed it is possible to wrap the integer type in a `newtype` and enforce another alignment via `#[align(N)]` | All integer types have the same `align_of` as the next biggest Rust built-in integer primitive. For example `align_of<ux::u24>() == align_of<u32> == 4`. 
| Focus | Data layout and alignment of packed data structures using integers. | Arithmetic operations on non-standard bitwidth integer types. |
| API | Integer types provide a minimal API surface. Mostly `From` and `TryFrom` impls between integers and Rust primitives as well as endian-aware byte conversions known from Rust primitives such as `to_ne_bytes` and `from_le_bytes`. | Integer types try to mimick Rust built-in integer types providing a fair amount of arithmetic operations on them. |

Both crates provide rich support for conversions between different integer types and Rust primitives.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
