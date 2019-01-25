#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn grow(&mut self, factor: &u32) {
        self.width = self.width * factor;
        self.height = self.height * factor;
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let mut rect1 = Rectangle { width: 30, height: 50 };
    let square1 = Rectangle::square(20);

    println!(
        "The area of the {:?} is {} square pixels.",
        &rect1,
        rect1.area()
    );

    println!(
        "Can the {:?} encompass {:?}? {}.",
        &rect1,
        &square1,
        rect1.can_hold(&square1)
    );

    let factor = 2;
    rect1.grow(&factor);

    println!(
        "After growing by factor of {} The area of the {:?} is {} square pixels.",
        &factor,
        &rect1,
        rect1.area()
    );
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
