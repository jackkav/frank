pub fn farenheit(celsius: i32) -> i32 {
    (((celsius as f32) - 32.) * 5. / 9.) as i32
}

pub fn bmi(weight: u32, height: f32) -> &'static str {
    let bmi = weight as f32 / height.powf(2.0);
    if bmi <= 18.5 {
        return "Underweight";
    }
    if bmi <= 25.0 {
        return "Normal";
    }
    if bmi <= 30.0 {
        return "Overweight";
    }
    "Obese"
}
