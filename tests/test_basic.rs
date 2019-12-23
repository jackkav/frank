use std::char;

#[test]
fn test_basic() {
    assert_eq!(2, 2);
    assert_eq!([2], [2]);
    assert_eq!("olleh", "hello".chars().rev().collect::<String>());
    assert_eq!("HELLO", "hello".to_uppercase());
    assert_eq!('2', "123".chars().nth(1).unwrap());
    // trimming
    assert_eq!("ello", &"hello"[1..]);
    assert_eq!("he", &"hello"[..2]);
    assert_eq!("e", &"hello"[1..2]);
    assert_eq!([2, 3], &[1, 2, 3][1..]);

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

    // binary
    assert_eq!(format!("{:b}", 42), "101010");
    assert_eq!(42, isize::from_str_radix("101010", 2).unwrap());
    // split string
    assert_eq!(
        vec!["some", "string", "123", "ffd"],
        "some string 123 ffd".split(' ').collect::<Vec<_>>()
    );
    // generate a list of numbers
    assert_eq!(vec![1, 2, 3], (1..4).collect::<Vec<_>>());
    // closure
    let greet = |name| String::from("Hello, ") + name;
    assert_eq!("Hello, Alice", greet("Alice"));
}
