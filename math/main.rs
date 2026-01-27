use std::io;

fn main() {
    let mut input_expression = String::new();
    let mut floats_present: bool = false;
    io::stdin().read_line(&mut input_expression);
    let mut input_expression: Vec<char> = input_expression.chars().collect();
    let mut shunting_yard_stack: Vec<char> = Vec::new();
    let mut shunting_yard_output: Vec<char> = Vec::new();
    let mut float_output: Vec<f64> = Vec::new();
    for symbol in input_expression {
        match symbol {
            "(" | ")" | "+" | "-" | "*" | "/" | "%" | "^" => shunting_yard_stack.push(symbol),
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                shunting_yard_output.push(symbol)
            }
            _ => (),
        };
    }
    for num in shunting_yard_output {
        let num: f64 = match num.parse() {
            Ok(num) => num,
            Err(num) => continue,
        };
        float_output.push(num);
    }
}

fn evaluate_expression(float_output: Vec<f64>, shunting_yard_stack: Vec<char>) {}
