
//All variables in Rust are inmutable by default

/*
Rules for ownership
1. Each value in Rust is owned by a variable
2. When the owner goes out of scope, the value will be deallocated
3. There can only be ONE owner at a time
*/

use std::io;

fn main() {
    // Compiler is smart enough to infer 
    // the type of this new variable

    println!("Enter your weight (kg):");

    let mut earth_weight = String::new();
    io::stdin().read_line(&mut earth_weight).unwrap();

    let earth_weight: f32 = earth_weight.trim().parse().unwrap();

    let mars_weight = calculate_weight_on_mars(earth_weight);
    // Every time we see a ! it is a macro
    println!("Weight on Mars: {}kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    // 50.0
    return (weight/9.81) * 3.711;
}



