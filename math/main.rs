use std::io;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let mut input_expression = String::new();
    io::stdin().read_line(&mut input_expression);
    // let mut input_expression: Vec<char> = input_expression.chars().collect();
    let mut shunting_yard_stack: Vec<&str> = Vec::new();
    let mut input_expression_nums: Vec<&str> = Vec::new();
    let mut expression_output: Vec<f64> = Vec::new();
    for symbol in input_expression.graphemes(true) {
        match symbol {
            "(" | ")" | "+" | "-" | "*" | "/" | "%" | "^" => shunting_yard_stack.push(symbol),
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                input_expression_nums.push(symbol)
            }
            _ => (),
        };
    }
    for num in input_expression_nums {
        let mut num: f64 = match num.parse() {
            Ok(num) => num,
            Err(num) => continue,
        };
        expression_output.push(num);
    }
}
