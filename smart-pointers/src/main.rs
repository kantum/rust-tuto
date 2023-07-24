use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }

// #[derive(Debug)]
// enum List {
//     Cons(Rc<RefCell<i32>>, Rc<List>),
//     Nil,
// }

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Droppinng CustomSmartPointer with data: `{}`", self.data);
    }
}

fn main() {
    // Needs List to have Box<List>
    // let _list =
    //     Box::new(Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))));

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    let m = MyBox::new(String::from("Jean Claude"));
    hello(&(*m)[..]);
    hello(&m);

    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    drop(_c);

    // Needs list to have Cons(i32, Rc<List>),
    // println!("CustomSmartPointers created.");
    // let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))));
    // println!("count after creating a = {}", Rc::strong_count(&a));
    // let _b = Rc::new(Cons(3, Rc::clone(&a)));
    // println!("count after creating b = {}", Rc::strong_count(&a));
    // {
    //     let _c = Rc::new(Cons(5, Rc::clone(&a)));
    //     println!("count after creating c = {}", Rc::strong_count(&a));
    // }
    // println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // Needs List to have Cons(Rc<RefCell<i32>>, Rc<List>),
    // let value = Rc::new(RefCell::new(5));
    //
    // let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    // let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    // let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    //
    // *value.borrow_mut() += 10;
    //
    // println!("a after = {:?}", a);
    // println!("b after = {:?}", b);
    // println!("c after = {:?}", c);
    //

    // Listing 15-26
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment next line to see cycle, it will overflow the stack
    // println!("a next item = {:?}", a.tail());

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}
