use minus_one::minus_one;

#[test]
fn test_minus_one() {
    let max_byte: u8 = minus_one!(256);
    assert_eq!(max_byte, 255);

    let min_byte: u8 = minus_one!(1);
    assert_eq!(min_byte, 0);
}