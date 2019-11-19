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
