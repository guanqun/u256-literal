//! This crate provides `u256!` macro for converting u256 literals to actual `primitive_types::U256` at compile time.
//! This is helpful when you're trying to define a const U256.
//!
//! It supports `0x` prefix and normal decimals. When the number is too large to fit into U256, it would complain.
//!
//! This crate assumes you have `primitive-types` added to your `Cargo.toml` dependency.
//!
//! # Examples
//! ```
//! # fn main() {
//! let x = u256!(123454444444444444444444444444444444444444444);
//! let y = u256!(0x123456789827823721235568483);
//! # }
//! ```

use proc_macro::TokenStream;
use syn::{parse_macro_input, LitInt};
use primitive_types::U256;

#[proc_macro]
pub fn u256(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitInt);
    let number = U256::from_dec_str(input.base10_digits()).expect("the number can't fit into U256");
    format!("primitive_types::U256([{}u64, {}u64, {}u64, {}u64])", number.0[0], number.0[1], number.0[2], number.0[3]).parse().unwrap()
}
