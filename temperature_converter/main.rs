use std::io;

fn main() {
    let mut mode = String::new();
    let mut input_temp = String::new();
    println!("Please input the temperature you would like to convert");
    io::stdin()
        .read_line(&mut input_temp)
        .expect("Invalid temperature input");
    let input_temp: f64 = match input_temp.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!(),
    };

    println!(
        "Please input what system you would like to conver this temperature to: 1 or c for Celsius, 2 or f for Farenheit"
    );
    io::stdin().read_line(&mut mode).expect("Invalid mode");
    match mode.trim().to_uppercase().as_str() {
        "1" | "C" => println!(
            "The converted temperature is {} :3",
            farenheit_to_celsius(input_temp)
        ),
        "2" | "F" => println!(
            "The converted temperature is {} :3",
            celsius_to_farenheit(input_temp)
        ),
        _ => panic!(
            "Could not match the given mode to a conversion function, instead received {}",
            mode
        ),
    };
}

fn farenheit_to_celsius(input_temp: f64) -> f64 {
    return (input_temp - 32.0) / 1.8;
}

fn celsius_to_farenheit(input_temp: f64) -> f64 {
    return (input_temp * 1.8) + 32.0;
}
