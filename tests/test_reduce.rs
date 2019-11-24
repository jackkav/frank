#[test]
fn test_reduce_or_fold() {
    assert_eq!(6, [1, 2, 3].iter().sum());
    assert_eq!(24, [1, 2, 3, 4].iter().product());
    assert_eq!(
        2i32,
        (123.to_string())
            .chars()
            .nth(1)
            .unwrap()
            .to_digit(10)
            .unwrap() as i32
    );
    assert_eq!(
        vec![1, 2, 3],
        vec![1, 1, 1, 1, 2, 2, 3, 3]
            .into_iter()
            .fold(Vec::new(), |mut v, i| {
                if let Some(last) = v.last() {
                    if last != &i {
                        v.push(i);
                    }
                } else {
                    v.push(i);
                }
                v
            })
    )
}
