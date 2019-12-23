#[test]
fn test_maps() {
    assert_eq!(true, [0, 1, 0].iter().any(|&x| x == 1));
    assert_eq!(true, [1, 1, 1].iter().all(|&x| x == 1));
    assert_eq!(
        vec![1, 2],
        vec![1, 2, 3, 4]
            .into_iter()
            .filter(|&s| s < 3)
            .collect::<Vec<_>>()
    );
    // filter none integers, Vec ~= List
    assert_eq!(
        vec![93, 18],
        vec!["tofu", "93", "18"]
            .into_iter()
            .map(|s| s.parse::<i32>())
            .filter_map(Result::ok)
            .collect::<Vec<_>>()
    );
}
