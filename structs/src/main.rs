struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        return self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 29,
        height: 10,
    };

    let rect3 = Rectangle {
        width: 40,
        height: 10,
    };

    println!("The area pixel is {} square pixels.",
        area(&rect1));
    dbg!(&rect1);
    println!("Rectangle: {:#?}", rect1);
    println!("Rectangle: {}", rect1.area());

    println!("It {} that rect1 can hold rect2", rect1.can_hold(&rect2));
    println!("It {} that rect1 can hold rect3", rect1.can_hold(&rect3));
}


fn area(rectangle: &Rectangle) -> i32 {
    return  rectangle.width * rectangle.height
}

fn area0(width: i32, height: i32) -> i32 {
    return  width * height
}

struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

struct AlwaysEqual;

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}

