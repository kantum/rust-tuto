#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32,i32,i32),
}

struct QuitMessage; // unit struct

struct MoveMessage {
    x: i32,
    y: i32,
}

struct WriteMessage(String); // tupple struct

struct ChangeColorMessage(i32,i32,i32); // tupple struct

impl Message {
    fn call(&self) {
        match self {
            Message::Move {x, y} => {
                println!("I'm moving to {x}, {y}");
            }
            Message::Write(s) => {
                println!("I'm writing {s}");
            }
            Message::ChangeColor(_, _, _) => {
                println!("I'm changing color");
            }
            _ => (),
        }
    }
}

fn main() {
    let m1 = Message::Write(String::from("hello"));
    let m2 = Message::Move{x: 24,y: 42};
    let m3 = Message::Quit;
    m1.call();
    m2.call();
    m3.call();

    let some_number = Some(5);
    let some_char  = Some('a');
    let absent_number: Option<i32> = None;

    let mut config_max = Some(3u8);

    if let Some(max) = config_max {
        println!("The max is {max}");
    } else {
        println!("there is no max");
    }


    match config_max {
        Some(max) => println!("The maximu is configured to be {max}"),
        _ => (),
    }
}
