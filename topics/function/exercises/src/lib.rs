pub fn mul(x: u32, y: u32) -> u32 {
    x * y
}

pub fn div(x, y) -> u32 {
    x / y
}

fn main() {
    let x: u32 = 3;
    let y: u32 = 5;

    println!("Multiplication of {x} and {y} is : {}", mul(x, y));
    println!("Division of {x} and {y} is : {}", div(x, y));
}
