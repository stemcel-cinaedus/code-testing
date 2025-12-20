use std::io;

fn main() {
    let mut cat = String::new();
    println!("How many cats are left? ");
    io::stdin()
        .read_line(&mut cat)
        .expect("That's not a valid number chuddie");
    let cat: u8 = match cat.trim().parse() {
        Ok(num) => num,
        Err(_) => u8::min_value(),
    };
    println!("{cat}")
}
