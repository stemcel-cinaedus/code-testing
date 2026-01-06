fn main() {
    let s = String::from("Hello, world!");
    println!("{}", first_word(&s));
}

fn first_word(input_string: &str) -> &str {
    let bytes = input_string.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &input_string[0..i];
        }
    }
    &input_string[..]
}
