use std::slice;

static HELLO_WORLD: &str = "Hello, world!";

// Changing static variables is unsafe.
static mut COUNTER: u32 = 0;

fn main() {
    let mut num = 5;

    // Creating raw pointers.
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // Arbitrary address
    let address = 0x012345usize;

    let _r = address as *const i32;

    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
        // println!("r is {}", *_r);
        // Call unsafe function.
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    // let (a, b) = r.split_at_mut(3);
    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // Run Foreign Function Interfaces (FFI) to execute C code.
    extern "C" {
        fn abs(input: i32) -> i32;
        fn sqrt(arg: f64) -> f64;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
        let num = 128;
        println!(
            "Square root value of {num} according to C: {}, according to rust: {}",
            sqrt(num as f64), (num as f64).sqrt()
        );
    }

    println!("static value is: {HELLO_WORLD}");

    add_to_count(4);

    unsafe {
        println!("COUNTER: {COUNTER}");
    }
}

// Usafe function.
unsafe fn dangerous() {
    println!("Beware la mouche !");
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    // This will raise an error from the borrow checker.
    // (&mut values[..mid], &mut values[mid..])

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}
