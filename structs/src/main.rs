#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    // let width1 = 30; //variables
    // let height1 = 50; //variables
    // let rect1 = (30, 50); //tuples
    let rect1 = Rectangle {
        height: 50,
        width: 30,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // println!(
    //     "The area of the rectangle {rect1:?} is {} square pixels",
    //     // area(width1, height1) //variables
    //     // area(rect1) // tuples
    //     // area(&rect1) // structs
    //     rect1.area()
    // );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

// fn area(width: u32, height: u32) -> u32 { // Variables#
// fn area(dimensions: (u32, u32)) -> u32 {
// fn area(rectangle: &Rectangle) -> u32 {
//     // width * height //variables
//     // dimensions.0 * dimensions.1 // tuples
//     rectangle.height * rectangle.width
// }
