fn main() {
    let number = 3;
    let condition = true;
    if number < 5 {
        println!("Number is less than 5")
    } else {
        println!("Number is greater than or equal to 5");
    }
    let pleiades = if condition { 7 } else { 6 };
    println!("pleiades is {pleiades}");
}
