#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn vectors() {
    // let mut v = vec![1, 2, 3, 4, 5];

    // for i in &mut v {
    //     *i *= 50;
    // }

    // v.push(0);

    // println!("{:#?}", v);

    let v = vec![
        SpreadsheetCell::Int(24),
        SpreadsheetCell::Float(23.201),
        SpreadsheetCell::Text(String::from("Hello world!")),
    ];

    for cell in &v {
        match cell {
            SpreadsheetCell::Float(f) => println!("This is a float: {}", f),
            SpreadsheetCell::Int(i) => println!("This is an integer: {}", i),
            SpreadsheetCell::Text(s) => println!("This is a string: {}", s),
        }
    }
}
