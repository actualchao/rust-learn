use crate::dev_log;
use std::collections::HashMap;

pub fn map() {
    dev_log("hashMap start");

    let mut test_map = HashMap::new();
    test_map.insert("test", "testV");

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    for _i in &mut scores {
        // 解构
        let (a, b) = _i;
        println!("value is {} : {}", a, b)
    }

    // s所有权发生变化， map 之后值不可用情况
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(&field_name, field_value);
    // error next line
    let some = &field_name;
    println!("some is {}", some);
    for _i in &mut map {
        // 解构
        let (a, b) = _i;
        println!("value is {} : {}", a, b);
    }

    // change Value
    map.insert(&field_name, String::from("Red"));
    println!("{:?}", map);

    let mut test_map = HashMap::new();

    test_map.insert(String::from("key"), 123);
    test_map.insert(String::from("key1"), 123);
    println!("{:?}", test_map);
    let entry = test_map.entry(String::from("key"));
    println!("{:?}", entry);

    dev_log("hashMap end");
}
