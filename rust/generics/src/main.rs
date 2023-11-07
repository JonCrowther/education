fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    fn mixup<X,Y>(self, other: Point<X,Y>) -> Point<T,Y> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['b', 'x', '3', 'e'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer = Point { x: 5, y:10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("integer.x = {}", integer.x());
    println!("float.x = {}", float.x());
    println!("Magnitude of float = {}", float.distance_from_origin());

    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p = p1.mixup(p2);
    println!("p.x = {}, p.y = {}", p.x, p.y);
}    
