#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        if self.width >= rect.width && self.height >= rect.height {
            return true;
        }
        false
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {

    let scale = 2;
    let rect = Rectangle {
        width: dbg!(2 * scale),
        height: 4,
    };

    let rect2 = Rectangle{
        width: 2,
        height: 5
    };

    let square = Rectangle::square(4);

    println!("The area of rectangle is {} square pixels.", rect.area());
    println!("The area of square is {} square pixels.", square.area());

    println!("Can rect hold rect2: {}", rect.can_hold(&rect2));


    dbg!(&rect);
}