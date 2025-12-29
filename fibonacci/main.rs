use std::io;

fn main() {
    let mut x: u64 = 0;
    let mut y: u64 = 1;
    let mut z: u64 = 0;
    let mut loop_exit: u64 = 0;
    let mut length = String::new();
    println!("Which digit of the fibonnaci sequence would you like to calculate?");
    io::stdin()
        .read_line(&mut length)
        .expect("Could not read length input.");
    let length: u64 = length
        .trim()
        .parse()
        .expect("Could not read the desired digit to calculate!");
    while loop_exit <= length {
        z = x + y;
        if x <= y {
            x = z
        } else {
            y = z
        }
        loop_exit += 1;
    }
    println!("Digit {length} of the fibonacci sequence is {z}")
}
