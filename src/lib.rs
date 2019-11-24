use std::error::Error;
use std::fs::read_to_string;

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
    let as_string = num.abs().to_string();
    if !nth.is_positive() {
        return -1;
    }
    if nth as usize > as_string.len() {
        return 0;
    }
    as_string
        .chars()
        .nth(as_string.len() - nth as usize)
        .unwrap_or('0')
        .to_digit(10)
        .unwrap() as i32
}

pub fn find_digit2(num: i32, nth: i32) -> i32 {
    if nth <= 0 {
        return -1;
    }
    let mut res = num;
    for _ in 1..nth {
        res /= 10;
    }
    res.abs() % 10
}

pub fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
where
    T: std::iter::IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
    let mut list: Vec<_> = sequence.into_iter().collect();
    list.dedup();
    list
}
pub fn high_and_low(numbers: &str) -> String {
    let n: Vec<i32> = numbers
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    format!("{} {}", n.iter().max().unwrap(), n.iter().min().unwrap())
}
pub fn read_file(path: &str) -> String {
    match read_to_string(path) {
        Err(err) => panic!("Couldn't read: {}", err.description()),
        Ok(data) => data,
    }
}
pub fn reverse_words(input: &str) -> String {
    let mut split_char = " ";
    // let mut two = " ";
    // for i in 9..1 {
    //     two = &" ".repeat(i).to_string();
    //     if String::from(input).contains(two) {
    //         split_char = two;
    //         break;
    //     }
    // }
    if String::from(input).contains("  ") {
        split_char = "  "
    }
    let n: Vec<String> = input
        .split(split_char)
        .map(|x| x.chars().rev().collect())
        .collect();
    n.join(split_char)
}
