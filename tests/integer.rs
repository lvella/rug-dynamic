#[test]
fn sum() {
    let a = rug_dynamic::Integer::from_str_radix("1000000000000000000000000000000000", 10).unwrap();
    let b = rug_dynamic::Integer::from(42);

    assert_eq!(
        a + b,
        rug_dynamic::Integer::from_str_radix("1000000000000000000000000000000042", 10).unwrap()
    );
}
