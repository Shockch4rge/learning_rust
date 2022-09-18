// fn main() {
//     let v4 = IpAddrKind::V4(127, 0, 0, 1);
//     let v6 = IpAddrKind::V6(String::from("localhost:8000"));

// }

// enum IpAddrKind {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// enum Message {
//     Quit,
//     Move {
//         x: i32,
//         y: i32,
//     },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// impl Message {
//     fn call(&self) {
//         //
//     }
// }

// fn main() {
//     let message0 = Message::Quit;

//     let message1 = Message::Move {
//         x: 32,
//         y: 2
//     };

//     let message2 = Message::Write(String::from("Hello world!"));

//     let message3 = Message::ChangeColor(123, 123, 123);

//     message0.call();
//     message1.call();
//     message2.call();
//     message3.call();
// }

// fn main() {
//     let penny = value_in_cents(Coin::Penny);
//     let nickel = value_in_cents(Coin::Nickel);
//     let dime = value_in_cents(Coin::Dime);
//     let quarter1 = value_in_cents(Coin::Quarter(UsState::Alabama));
//     let quarter2 = value_in_cents(Coin::Quarter(UsState::Alaska));
// }

// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => match state {
//             UsState::Alabama => {
//                 println!("wtf are you doing here");
//                 25
//             }
//             UsState::Alaska => {
//                 println!("this place sucks");
//                 0
//             }
//         },
//     }
// }

// fn main() {
//     let maybe_two = plus_one(Some(1));
// }

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }

// fn main() {
//     let dice_roll = 9;
//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         _ => ()
//     }
// }

// fn add_fancy_hat() {}
// fn remove_fancy_hat() {}

fn main() {
    let config_max = Some(3u8);

    if let Some(max) = config_max {
        println!("The maximum is configured to be: {}", max);
    }
    else {
        println!("Bruh");
    }
}

