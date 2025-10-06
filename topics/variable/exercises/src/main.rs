#![allow(unused)]
// Exercise: Fix the code to make it compile and pass the assertions

fn main() {
    // Exercise 1: Make this variable mutable
    let mut count = 1;
    count += 1;

    println!("count: {count}");

    // BUT, you can also change it by redeclaring it
    let count2 = 1;
    let count2 = count2 + 1;
    println!("count2: {count2}");
}
