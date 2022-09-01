#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other : &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
    fn square(size: u32) -> Self {
        Self { length: size, width: size }
    }
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };
    let rect2 = Rectangle { length: 40, width: 10 };
    let rect3 = Rectangle { length: 45, width: 60 };
    let rect4 = Rectangle::square(50);

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    }
}