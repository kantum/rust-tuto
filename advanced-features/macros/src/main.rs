use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

// Declarative macro
#[macro_export]
macro_rules! veco {
    ( $( $x:expr ), * ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
                temp_vec}
    };
}

#[derive(HelloMacro)]
struct Pancakes;

#[derive(HelloMacro)]
struct Crepes;

fn main() {
    let v: Vec<u32> = veco![1, 2, 3];

    println!("{:?}", v);

    Pancakes::hello_macro();
    Crepes::hello_macro();
}
