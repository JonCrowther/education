#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width > rectangle.width && self.height > rectangle.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30*scale),
        height: 50,
    };
    let rect2 = Rectangle {
        width: 40,
        height: 30,
    };
    let square1 = Rectangle::square(20);

    println!("Area of rectangle is {}", rect1.area());
    println!("Can rect1 fit rect2? {}", rect1.can_hold(&rect2));
    println!("Area of square1 is {}", square1.area());

    println!("rect1 is {:#?}", rect1);
    dbg!(&rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
