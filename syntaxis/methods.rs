#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// All functions defined within an impl block are called associated functions because they’re associated with the type named after the impl.
impl Rectangle {
    // A simple method
    fn area(&self) -> u32 {
        self.width * self.height
    }

}

// Each struct is allowed to have multiple impl blocks.
impl Rectangle {
    // A simple method with params
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // We can define associated functions that don’t have self as their first parameter (and thus are not methods) because they don’t need an instance of the type to work with.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 23,
        height: 30,
    };

    let square = Rectangle::square(3);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!(
        "R1 can hold R2? {}",
        rect1.can_hold(rect2)
    );
}
