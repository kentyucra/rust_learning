fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);

    println!("square = {:?}", sq);
}

fn using_single_variables() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The are of the rectangle is {} square pixels.",
        area1(width1, height1)
    );
}

fn using_tuples() {
    let rect1 = (30, 50);

    println!(
        "The are of the rectangle is {} square pixels.",
        area2(rect1)
    );    
}

fn using_structs() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The are of the rectangle is {} square pixels.",
        rect1.area()
    ); 

    println!(
        "Struct: {:#?}",
        rect1
    );
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn area1(width: u32, height: u32) -> u32 {
    return width*height;
}

fn area2(dimensions: (u32, u32)) -> u32 {
    return dimensions.0 * dimensions.1;
}

fn area3(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
}

