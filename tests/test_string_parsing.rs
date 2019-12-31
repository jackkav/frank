#[test]
fn test_string_parsing() {
    assert_eq!(
        alphabet_position("The sunset sets at twelve o' clock."),
        "20 8 5 19 21 14 19 5 20 19 5 20 19 1 20 20 23 5 12 22 5 15 3 12 15 3 11".to_string()
    );
    assert_eq!(
        alphabet_position("The narwhal bacons at midnight."),
        "20 8 5 14 1 18 23 8 1 12 2 1 3 15 14 19 1 20 13 9 4 14 9 7 8 20".to_string()
    );
    assert_eq!(alphabet_position("abc"), "1 2 3".to_string());
    assert_eq!(alphabet_position("xyz"), "24 25 26".to_string());
    assert_eq!(
        alphabet_position("abcdefghijklmnopqrstuvxyz"),
        "1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 24 25 26".to_string()
    );
    // assuming a is 0
    assert_eq!(20, from_a('T' as u8));
    assert_eq!(8, from_a('h' as u8));
    assert_eq!(5, from_a('e' as u8));
    assert_eq!(1, from_a('a' as u8));
    assert_eq!(26, from_a('z' as u8));
    assert_eq!(1, from_a('A' as u8));
    assert_eq!(26, from_a('Z' as u8));
}
fn from_a(letter: u8) -> u8 {
    return letter
        - match letter {
            65..=90 => b'A',
            97..=122 => b'a',
            _ => b'a',
        }
        + 1;
}
fn is_letter(letter: u8) -> bool {
    return match letter {
        65..=90 => true,
        97..=122 => true,
        _ => false,
    };
}
fn alphabet_position(text: &str) -> String {
    text.chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| (c.to_ascii_uppercase() as u8 - 64).to_string())
        .collect::<Vec<String>>()
        .join(" ")
}
