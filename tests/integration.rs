use primitive_types::U256;
use u256_literal::u256;

#[test]
fn test_integer() {
    assert_eq!(u256!(1234567890), U256::from_dec_str("1234567890").unwrap());
}

#[test]
fn test_float() {
    assert_eq!(u256!(123.45678e5), U256::from_dec_str("12345678").unwrap());
    assert_eq!(u256!(123.45e5), U256::from_dec_str("12345000").unwrap());
    assert_eq!(u256!(123e5), U256::from_dec_str("12300000").unwrap());
    assert_eq!(u256!(123.), U256::from_dec_str("123").unwrap());
}
