// code samples from the Rust Book V. 2 Chap. 3
fn main() {
    // learning about shadowing
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The number of spaces is: {}", spaces);

    // different base declarations and seperators
    let dec = 42_000;
    println!("The value of the number dec is: {}", dec);

    let hex = 0xff;
    println!("The value of the hex number is: {}", hex); // in dec
    println!("The value of the hex number is: {}", format!("{:X}", hex)); // upper hex

    // floating point numbers
    let floating = 2.20;

    // bool
    let f: bool = false;

    // char (is unicode formatted, read Emoji)
    let c = 'c';

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup; //destructuring
    println!("The second value of the tuple is: {}", b);
    println!("The third value of the tuple is: {}", tup.2);

    // arrays
    let d = [1, 2, 3, 4, 5];
    let first = d[0];
}
