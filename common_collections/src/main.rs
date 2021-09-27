use std::error::Error;
use std::vec;

fn vectors() {

    // Basic operations on Vectors
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(5);
    v.push(6);
    v.push(5);
    v.push(6);

    let v = vec![1,2,3,4,5];

    let thrid: &i32 = &v[2];
    println!("The thrid element is {}", thrid);

    match v.get(2) {
        Some(third) => println!("The thrid element is {}", thrid),
        None => println!("THere is not third element."),
    }

    // Iterating over the values in a Vector

    let v = vec![100, 32, 57];

    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    // Using an Enum to Store Multiple Types

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

}

// fn vector_with_problem() {
//     let mut v = vec![1,2,3,4,5];

//     let first = &v[0];

//     v.push(6);

//     // We can not access here because if we push a element in the vector
//     // and there is not space we will have to move all the vector to 
//     // other part of the heap memory and the first element is in another place (in the heap)
//     println!("the first element is: {}", first);
// }

fn strings() {
    // Initialization ways
    let mut s = String::new();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    // Strings are UTF-8 enconded
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // Appending to a string with push_str and push
    let mut s1 = String::from("foo");
    let s2 = "bar";
    // push_str work with a str
    s1.push_str(s2);
    println!("s1 = {}", &s1);
    // push work with a single character 
    s1.push('+');
    println!("s1 = {}", &s1);

    // Concatenation with the + Operator or the format! Macro
    // Signature of the + Operator: **fn add(self, s: &str) -> String {**
    // Example 1
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("s3 = {}", s3);
    // Example 2
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s = {}", s);
    // Example 3 using **format!** macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s = {}", s);

    // Indexing into Strings: **indexing syntax in Rust DOES NOT work**
    // Follow code does not compile because how strings are stored in Rust
    // let s1 = String::from("hello");
    // let h = s1[0];
    // Example1
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("s = {}", s);

    // Methods for Iterating over strings
    // Example 1: Using chars
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    // Example 2: Using bytes
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}

use std::collections::HashMap;

fn hash_maps() {
    // First way to create and fill a hash map
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Second way to create and fill a hash map
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> = 
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    // Accessing values in a hash map
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("team_name = {}", &team_name);
    match score {
        Some(valid_score) => println!("Score: {}", valid_score),
        None => println!("There is not element"),
    }

    // Printing key ans values from a hash map
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Updating a hash map
    // Overwriting a value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);
    
    // Only inserting value if the key has no value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // Updating a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

fn main() {
    println!("Hello, world!");

    hash_maps();
}
