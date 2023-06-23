use std::io;

fn fib(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}

fn main() {
    println!("Fibonacci");
    println!("Give a number");

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: u32 = match number.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            println!("Not a number ({e})");
            return;
        }
    };

    let result = fib(number);
    println!("{}", result);
}
