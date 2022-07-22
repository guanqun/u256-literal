//! This crate provides `u256!` macro for converting u256 literals to actual `primitive_types::U256` at compile time.
//! This is helpful when you're trying to define a const U256.
//!
//! It supports `0x` prefix and normal decimals. When the number is too large to fit into U256, it would complain.
//!
//! This crate assumes you have `primitive-types` added to your `Cargo.toml` dependency.
//!
//! # Examples
//! ```
//! # use u256_literal::u256;
//! # fn main() {
//! let x = u256!(123454444444444444444444444444444444444444444);
//! let y = u256!(0x123456789827823721235568483);
//! let z = u256!(1e18);
//! let w = u256!(1.23e18);
//! # }
//! ```

use primitive_types::U256;
use proc_macro::TokenStream;
use std::str::FromStr;
use syn::{parse_macro_input, Lit};

#[proc_macro]
pub fn u256(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Lit);
    let number = match input {
        Lit::Int(lit) => parse_int(lit.base10_digits()),
        Lit::Float(lit) => parse_float(lit.base10_digits()),
        _ => panic!("only integer and float are allowed!"),
    };

    format!(
        "primitive_types::U256([{}u64, {}u64, {}u64, {}u64])",
        number.0[0], number.0[1], number.0[2], number.0[3]
    )
    .parse()
    .unwrap()
}

fn parse_int(s: &str) -> U256 {
    U256::from_dec_str(s).expect("the number can't fit into U256")
}

fn parse_float(s: &str) -> U256 {
    let dot_pos = s.find(".");
    let e_pos = s.find("e");
    match (dot_pos, e_pos) {
        (Some(dot_pos), Some(e_pos)) => {
            assert!(dot_pos < e_pos);
            // 123.459e89
            let fraction_len = s[dot_pos + 1..e_pos].len();
            let exp = usize::from_str(&s[e_pos + 1..]).unwrap();
            if fraction_len > exp {
                panic!("don't support fractions");
            }

            let mut expanded = s[0..dot_pos].to_string();
            expanded.push_str(&s[dot_pos + 1..e_pos]);
            for _ in 0..(exp - fraction_len) {
                expanded.push('0');
            }

            parse_int(expanded.as_str())
        }
        (None, Some(e_pos)) => {
            // 123e34
            let exp = usize::from_str(&s[e_pos + 1..]).unwrap();
            let mut expanded = s[0..e_pos].to_string();
            for _ in 0..exp {
                expanded.push('0');
            }
            parse_int(expanded.as_str())
        }
        (Some(dot_pos), None) => {
            // only allowed format is 1234.
            if !s[dot_pos + 1..].is_empty() {
                panic!("can't handle number with fractions")
            } else {
                U256::from_dec_str(&s[0..dot_pos]).expect("the number can't fit into u256")
            }
        }
        _ => unreachable!("it should be covered in LitInt case"),
    }
}
