fn main() {
    let mut s = String::from("hi");

    let r1 = &mut s;
    // let r2 = &mut s; You can't do a double mutable reference!
    println!("{r1}");
}
