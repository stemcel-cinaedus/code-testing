use std::io;
//use core::f64::math::sqrt;
//use core::panicking;
//struct polynomial(i32, i32, i32);

fn main() {
    let mut input = String::new();
    //let mut a: i32 = 0;
    //let mut b: i32 = 0;
    //let mut c: i32 = 0;
    println!("Input a, b, and c of the quadratic polynomial, seperated by spaces.");
    io::stdin().read_line(&mut input).expect("Couldn't read input.");
    let mut str_vals: Vec<&str> = input.split(" ").collect();
    let mut quadratic_vals = vec!();
    for string_value in str_vals {
        let num: f64 = match string_value.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        quadratic_vals.push(num);
     }
     let solved = quadratic_formula(quadratic_vals[0], quadratic_vals[1], quadratic_vals[2]);
     println!("The roots are {} and {}", solved.0, solved.1)

}

fn quadratic_formula(a: f64, b: f64, c: f64) -> (f64, f64) {
    let root1 = (-b + f64::sqrt((b * b) * f64::from(4) * a * c)) / (f64::from(2) * a);
    let root2 = (-b - f64::sqrt((b * b) * f64::from(4) * a * c)) / (f64::from(2) * a);
    
    return(root1, root2)

}
