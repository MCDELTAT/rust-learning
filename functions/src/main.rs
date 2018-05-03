// Code examples for Rust Book V2 Chapter 3 Sec. 3
fn main() {
    println!("Hello, world!");

    another_function(5, 7);
    println!("The return of the five function is: {}", five(4));
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five(x: i32) -> i32 {
    x + 1
}
