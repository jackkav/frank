extern crate frank;
#[test]
fn test_farenheit() {
    assert_eq!(10, frank::farenheit(50));
    assert_eq!(22, frank::farenheit(72))
}
