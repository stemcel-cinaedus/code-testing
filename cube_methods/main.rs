use std::io;

#[derive(Debug)]
struct Cube {
    height: u32,
    width: u32,
    depth: u32,
}

impl Cube {
    fn volume(&self) -> u32 {
        self.height * self.width * self.depth
    }
}

fn main() {
    let mut dimensions = String::new();
    let mut dimensionArray: [u32; 3] = [0,0,0];
    println!("What are the dimensions of your cube? Enter height, width and depth, separated by spaces.");
    io::stdin().read_line(&mut dimensions).expect("Failed to read input.");
    let dimensions: Vec<&str> = dimensions.trim().split(' ').collect();
    let mut i: usize = 0;
    
    loop {
        let dimension = dimensions[i];
        let dimension: u32 = match dimension.parse() {
            Ok(num) => { num },
            Err(_) => break,
        };
        dimensionArray[i] = dimension;
        
        if i >= 2 {
            break;
        }
        i += 1;
    }
    
    let cube1 = Cube {
        height: dimensionArray[0],
        width: dimensionArray[1],
        depth: dimensionArray[2],
    };
    
    println!("{}", cube1.volume());
    
    
}
