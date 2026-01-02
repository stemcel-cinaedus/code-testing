fn main() {
    let s = String::from("Hiiiii :3");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);
    println!("{x} Printed again from within the main function, just to prove it :3")
}

fn takes_ownership(input_string: String) {
    println!(
        "The contents of the following are from a string that is now owned by this function: {input_string}"
    )
}

fn makes_copy(input_int: i32) {
    println!(
        "Because the integer {input_int} is stored on the stack, printing it out in this function doesn't drop the value passed into it :p"
    )
}
