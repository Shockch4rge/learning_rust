// fn main() {
//     let user1 = build_user(String::from("hello@gmail.com"), String::from("Shock"));

//     let user2 = User {
//         email: String::from("world@gmail.com"),
//         ..user1
//     };

//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         email,
//         username,
//         sign_in_count: 1,
//     }
// }

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }

    fn square(size: i32) -> Self {
        Self {
            height: size,
            width: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 30,
        height: 40,
    };

    println!("The area of the rectangle is {} pixels", &rect1.area());
    println!("rect1 can hold rect2: {}", &rect1.can_hold(&rect2));
    dbg!(Rectangle::square(20));
}
