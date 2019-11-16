#[test]
fn test_basic() {
    assert_eq!(2, 2);
    assert_eq!([2], [2]);
    assert_eq!("olleh", "hello".chars().rev().collect::<String>());
    assert_eq!("HELLO", "hello".to_uppercase());
    // trimming
    assert_eq!("ello", &"hello"[1..]);
    assert_eq!("he", &"hello"[..2]);
    assert_eq!("e", &"hello"[1..2]);
    assert_eq!([2, 3], &[1, 2, 3][1..]);
    // reduce
    assert_eq!(6, [1, 2, 3].iter().fold(0, |total, next| total + next));
    assert_eq!(24, [1, 2, 3, 4].iter().fold(1, |total, next| total * next));

    // filter
    assert_eq!(true, [0, 1, 0].iter().any(|&x| x == 1));
    assert_eq!(true, [1, 1, 1].iter().all(|&x| x == 1));
    // filter none integers, Vec ~= List
    assert_eq!(
        vec![93, 18],
        vec!["tofu", "93", "18"]
            .into_iter()
            .map(|s| s.parse::<i32>())
            .filter_map(Result::ok)
            .collect::<Vec<_>>()
    );
    // options
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;
    let first = slice.get(0);
    let last = slice.get(5);
    assert!(first.is_some());
    assert!(last.is_none());

    //guard
    assert_eq!(1, *first.unwrap_or(&-1));
    assert_eq!(-1, *last.unwrap_or(&-1));
    // map
    assert_eq!(
        vec![2, 2, 2],
        vec![1, 1, 1].into_iter().map(|x| x + 1).collect::<Vec<_>>()
    );
}
