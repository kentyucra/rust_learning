// Example 1

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn example1() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("email: {}, username: {}, active: {}, sign_in_count: {}", 
        user1.email, 
        user1.username, 
        user1.active, 
        user1.sign_in_count);
    
    user1.email = String::from("anotheruser@example.com");
}

// Example 2

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    }
}

fn example2() {
    let coin = Coin::Penny;
    let value = value_in_cents(&coin);
    
    println!("{:?} value in cents is {}", &coin, value);

    let coin = Coin::Quarter(UsState::Alaska);
    let value = value_in_cents(&coin);

    println!("{:?} value in cents is {}", &coin, value);
    
}

// Example 3

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

// Match is exhaustive
// fn plus_one1(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(i) => Some(i + 1),
//     }
// }


fn example3() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("result = {:?}", six);

    println!("result = {:?}", none);
}

fn main() {
    
    example3();    

}


