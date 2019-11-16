extern crate frank;
#[test]
fn test_farenheit() {
    assert_eq!(10f32, frank::farenheit(50f32))
}
