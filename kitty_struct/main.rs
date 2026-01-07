use std::io;

struct Kitty {
    alive: bool,
    fur_color: String,
    name: String,
    leg_count: u8,
}

fn main() {
    let mut kitty1 = Kitty {
        alive: true,
        fur_color: String::from("Calico"),
        name: String::from("Mary Magdalene"),
        leg_count: 4,
    };

    let mut collection = vec![];
    loop {
        let mut new_cat_name = String::new();
        let mut new_cat_fur = String::new();
        let mut answer = String::new();
        
        println!("What would you like to name your new cat?");
        let mut new_cat_name = io::stdin().read_line(&mut new_cat_name).expect("Failed to read name.");
        let new_cat_name = new_cat_name.to_string();
        println!("What color is its fur?");
        let new_cat_fur = io::stdin().read_line(&mut new_cat_fur).expect("Failed to read fur color.");
        let new_cat_fur = new_cat_fur.to_string();
        collection.push(birth_kitten(new_cat_fur, new_cat_name));
        
        println!("Would you like to keep adding cats? Answer with Y or N");
        let mut answer = io::stdin().read_line(&mut answer).expect("Couldn't read input");
        let mut answer = answer.to_string();
        let answer = answer.trim().to_uppercase();
        match &answer[..] {
            "Y" => continue,
            "N" => break,
            _ => println!("Invalid input"),
        }
    }

}

fn birth_kitten(fur_color: String, name: String) -> Kitty {
    Kitty {
    alive: true, //There are no stillbirths in this program
    fur_color,
    name,
    leg_count: 4 //No mutants or malformed kittens either
    }
}
