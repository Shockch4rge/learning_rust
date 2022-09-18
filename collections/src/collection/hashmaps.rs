use std::collections::HashMap;

pub fn hashmaps() {
    // let mut scores: HashMap<String, u8> = HashMap::new();

    // scores.insert(String::from("deez"), 8);
    // scores.insert(String::from("nuts"), 16);

    // scores.entry(String::from("deez")).or_insert(24);

    // println!("{:#?}", scores);

    let text = "hello world wonderful world";
    let mut m: HashMap<&str, u8> = HashMap::new();

    for word in text.split_whitespace() {
        let count = m.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", m)
}
