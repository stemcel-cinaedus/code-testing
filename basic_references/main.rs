fn main() {
    let s1 = String::from("haiiiiii");
    let len = calculate_len(&s1);
    println!("The length of {s1} is {len}"); //Because our length function only borrows s1, we can still use it after calling the function with it.
}

fn calculate_len(input_str: &String) -> usize {
    input_str.len()
    // input_str.push_str(", hru????") //We cannot modify the input string because it is borrowed, and it's a dick move to modify something you were lended.
}
