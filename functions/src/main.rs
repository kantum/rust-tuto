fn main() {
    println!("Hello, world!");
    another_function(4);
    print_labeled_measurement(5, 'h');
    println!("{}", five().1);
}

fn another_function(x: i32) {
    println!("Another function.");
    println!("The value of  x  is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn five() -> (i32, i64) {
    (5, 42)
}
