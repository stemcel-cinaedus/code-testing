fn main() {
    let s = "hello :3";
    println!("{}", s);
    let mut s = String::from("hello :3"); // The double colon operator is syntax that lets us take the `from` method as belonging to String instead of using a method name like `from_string`
    println!("{s}");
    let s2 = s.clone();
    s.push_str(", my name is rust"); //push_str is a method to append a string literal to a String
    println!("{s}");
    println!("{s2}");
}
