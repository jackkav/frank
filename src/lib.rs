pub fn farenheit(celsius: i32) -> i32 {
    (((celsius as f32) - 32.) * 5. / 9.) as i32
}
