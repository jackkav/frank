use std::collections::BTreeMap;

#[test]
fn test_hash_map() {
    let mut map = BTreeMap::new();
    map.insert("iPhone", "Apple");
    map.insert("Galaxy", "Samsung");
    assert_eq!(vec![&"Galaxy", &"iPhone"], map.keys().collect::<Vec<_>>());
    assert_eq!(vec![&"Samsung", &"Apple"], map.values().collect::<Vec<_>>())
}
