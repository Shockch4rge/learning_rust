#[derive(Debug)]
struct Point<X, Y> {
    x: X,
    y: Y,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    println!("Hello, world!");

    let p1 = Point {
        x: 1, 
        y: 1,
    };

    let p2 = Point {
        x: 2,
        y: 2,
    };

    let p3 = p1.mixup(p2);

    
    println!("{:#?}", p3);
}
