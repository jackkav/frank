#[test]
fn test_read_file() {
    assert_eq!(
        true,
        frank::read_file("Cargo.toml").starts_with("[package]\nname = \"frank\"")
    )
}

#[test]
fn test_parse_input() {
    assert_eq!(vec!["/target", "**/*.rs.bk"], frank::parse_input(".gitignore"))
}
