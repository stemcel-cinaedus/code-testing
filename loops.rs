use std::io;

fn main() {
    let mut loop_count = 0;
    let result = loop {
        loop_count += 1;
        println!("This text is being looped over :O");
        if loop_count == 12 {
            break loop_count * 10;
        }
    };
    println!("The result is {result}");

    //completely different exercises lol

    let mut number = String::new();
    println!("How long should the countdown be? ");
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read input");
    let mut number: u32 = number
        .trim()
        .parse()
        .expect("Please insure the coundown is a positive number!");
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("Liftoff!!");

    let uwu = [20, 40, 60, 80, 100];
    let mut index = 0;

    while index < 5 {
        println!("The value of uwu is {}", uwu[index]);
        index += 1;
    }
}
