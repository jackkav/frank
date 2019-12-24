#[test]
fn test_maps() {
    // increment
    assert_eq!(
        vec![2, 2, 2],
        vec![1, 1, 1].into_iter().map(|x| x + 1).collect::<Vec<_>>()
    );
    // char to int
    assert_eq!(97, 'a' as u8);
    // assuming a is 0
    assert_eq!(8, ('h' as u8)-b'a'+1);
    assert_eq!(
        vec![97, 98, 99, 100, 101],
        vec!['a', 'b', 'c', 'd', 'e']
            .into_iter()
            .map(|x| x as u8)
            .collect::<Vec<u8>>()
    );
    // int to ascii char
    assert_eq!('a', 97 as char);
    assert_eq!(b'a', 97);
    assert_eq!(
        vec!['a', 'b', 'c', 'd', 'e'],
        (0..5).map(|x| (x + b'a') as char).collect::<Vec<char>>()
    );
}
