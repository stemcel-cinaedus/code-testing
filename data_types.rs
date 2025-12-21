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
    let x = 5.6; //f64 is default float
    let y: f32 = 3.0; //32 bit float
    let z = x + y;
    let demo_tuple: (i64, u64, char, f32) = (-45, 2323, '$', 420.69);
    let demo_array = [1, 2, 3, 4, 5];
    // println!("{demo_tuple.1}");
    // println!("{demo_array[1]}");

    println!("{z}");
    println!("{cat}")
}
