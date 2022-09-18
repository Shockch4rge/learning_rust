pub fn strings() {
    // let mut s = String::from("Hello");
    // s.push_str(" world!");

    // println!("{:?}", s)

    // let s1 = String::from("hello");
    // let s2 = String::from(" world");

    // // s3 basically takes ownership of s1 and appends s2, not creating a new one by copying and concating s1 and s2
    // let s3 = s1 + &s2;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("{:?}", s);
}