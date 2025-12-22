fn main() {
    println!("Hello, world!");

    another_function();

    third_cooler_function(13);
}

fn another_function() {
    println!("This text comes from outside the main function!");
}

fn third_cooler_function(variable_name: isize) {
    let variable_name = variable_name * variable_name;
    println!("{variable_name}")
}
