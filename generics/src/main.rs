use std::{cmp, panic::resume_unwind};

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        largest = cmp::max(largest, item);
    }

    return largest;
}

fn largest_char(list: &[char]) ->  char {
    let mut largest = list[0];

    for &item in list {
        largest = cmp::max(largest, item);
    }

    return largest;
}

// ----------------------- In Functions Definitions
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

// ----------------------- In Struct definitions
struct Point<T, U> {
    x: T,
    y: U,
}

// ----------------------- In Method Definition
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        return &self.x;
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        return Point { 
            x: self.x, 
            y: other.y, 
        }
    }
}

// ----------------------- In Enum Defintions
// Example 1
enum Option<T> {
    Some(T),
    None,
}

// Example 2:
enum Result<T, E> {
    Ok(T),
    Err(E),
}


fn main() {

    let number_list = vec![34, 50, 23, 123, 456, 1];
    let result = largest_i32(&number_list);
    println!("Result = {}", result);

    let chart_list = vec!['y', 'a'];
    let result = largest_char(&chart_list);
    println!("Ressult = {}", result);

    let integer = Point {x:5, y:6.0};
    let integer = Point {x:6, y:5.0};

    println!("x = {}", integer.x());
    println!("y = {}", integer.y);

    let p1 = Point {
        x: 5,
        y: 10.4
    };
    let p2 = Point {
        x: "Hello",
        y: 'c'
    };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
