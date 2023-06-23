use std::any::type_name;

fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("the result is {result}");

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
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("Lift off!");

    let a = [43, 32, 4, 2, 2345, 3098, 23443];

    for element in a {
        println!("{}", element);
    }

    for number in (1..6).rev() {
        println!("{number}");
    }
}
