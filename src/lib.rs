pub fn farenheit(celsius: i32) -> i32 {
    (((celsius as f32) - 32.) * 5. / 9.) as i32
}

pub fn bmi(weight: u32, height: f32) -> &'static str {
    match weight as f32 / height.powf(2.0) {
        x if x <= 18.5 => "Underweight",
        x if x <= 25. => "Normal",
        x if x <= 30. => "Overweight",
        _ => "Obese",
    }
}

pub fn find_digit(num: i32, nth: i32) -> i32 {
    let mut n = num;
    if n.is_negative() {
        n = num.wrapping_abs()
    }
    if !nth.is_positive() {
        return -1;
    }
    let length = n.to_string().len();
    if nth as usize > length {
        return 0;
    }
    let right_to_left: usize = length - nth as usize;
    let nth_char = n.to_string().chars().nth(right_to_left).unwrap_or('0');
    nth_char.to_digit(10).unwrap() as i32
}
