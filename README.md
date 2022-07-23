# u256-literal

## Description

This crate helps you to declare a constant `U256` in an easy way. No need to wrap the integer literal around double quotes.

## Usage

```
use primitive_types::U256;
use u256_literal::u256;

const Q96: U256 = u256!(0x1000000000000000000000000);

// support hex style integer
let x = u256!(0x1234ff);
// support normal style
let y = u256!(987654);
// support float style number
let z = u256!(1.23e18);
```

## License

All crates licensed under either of

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
