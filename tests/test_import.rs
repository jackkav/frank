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
