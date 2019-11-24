#[test]
fn test_read_file() {
    assert_eq!(
        true,
        frank::read_file("Cargo.toml").starts_with("[package]\nname = \"frank\"")
    )
}
