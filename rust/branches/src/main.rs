fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("number is {number}");

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter + 2;
        }
    };

    println!("result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("end count: {count}");

    let mut countdown = 3;
    while countdown != 0 {
        println!("{countdown}");
        countdown -= 1;
    }
    println!("LIFTOFF");

    let a = [10,20,30,40,50];
    for element in a {
        println!("element is {element}");
    }
}
