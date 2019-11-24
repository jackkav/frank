extern crate frank;
#[test]
fn test_farenheit() {
    assert_eq!(10, frank::farenheit(50));
    assert_eq!(22, frank::farenheit(72))
}

#[test]
fn test_bmi() {
    assert_eq!(frank::bmi(50, 1.80), "Underweight");
    assert_eq!(frank::bmi(80, 1.80), "Normal");
    assert_eq!(frank::bmi(90, 1.80), "Overweight");
    assert_eq!(frank::bmi(110, 1.80), "Obese");
}

#[test]
fn test_find_digit() {
    assert_eq!(frank::find_digit(5673, 4), 5);
    assert_eq!(frank::find_digit(129, 2), 2);
    assert_eq!(frank::find_digit(-2825, 3), 8);
    assert_eq!(frank::find_digit(-456, 4), 0);
    assert_eq!(frank::find_digit(0, 20), 0);
    assert_eq!(frank::find_digit(65, 0), -1);
    assert_eq!(frank::find_digit(24, -8), -1);
}

#[test]
fn test_find_digit2() {
    assert_eq!(frank::find_digit2(5673, 4), 5);
    assert_eq!(frank::find_digit2(129, 2), 2);
    assert_eq!(frank::find_digit2(-2825, 3), 8);
    assert_eq!(frank::find_digit2(-456, 4), 0);
    assert_eq!(frank::find_digit2(0, 20), 0);
    assert_eq!(frank::find_digit2(65, 0), -1);
    assert_eq!(frank::find_digit2(24, -8), -1);
}
#[test]
fn test_unique_in_order() {
    assert_eq!(
        frank::unique_in_order("AAAABBBCCDAABBB".chars()),
        vec!['A', 'B', 'C', 'D', 'A', 'B']
    );
}
#[test]
fn test_high_and_low() {
    assert_eq!("42 -9", frank::high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"))
}
#[test]
fn test_reverse_words() {
    assert_eq!(
        frank::reverse_words("The quick brown fox jumps over the lazy dog."),
        "ehT kciuq nworb xof spmuj revo eht yzal .god"
    );
    assert_eq!(frank::reverse_words("apple"), "elppa");
    assert_eq!(frank::reverse_words("a b c d"), "a b c d");
    assert_eq!(
        frank::reverse_words("double  spaced  words"),
        "elbuod  decaps  sdrow"
    );
}
