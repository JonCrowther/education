fn main() {
    print_labeled_measurement(5, 'h');
    function2();

    let x = plus_one(5);
    println!("the value of x is {x}");
}

fn print_labeled_measurement(x: i32, unit_label: char) {
    println!("the measurement is {x}{unit_label}");
}

fn function2() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}