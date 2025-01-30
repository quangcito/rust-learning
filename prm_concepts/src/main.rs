use std::io;

fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    let x = 6;
    println!("The value of x is: {x}");


    const FIVE: u32 = 5;

    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("The value of five_hundred is: {five_hundred}");

    // Unlike a tuple, every element of an array must have the same type.
    // Arrays in Rust have a fixed length, like tuples.
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    println!("The value of a[0] is: {first}");

    let a = [3; 5];
    let fifth = a[4];
    println!("The value of a[4] is: {fifth}");

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {index} is: {element}"
    );

    let x = plus_one(5);

    println!("The value of x is: {x}");

    let a = [1, 2, 3, 4, 5];

    for element in a {
        println!("the value is: {element}");
    }
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
