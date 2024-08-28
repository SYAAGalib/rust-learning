struct Rectangle{
    hight: u32,
    width: u32,
}
impl Rectangle{

    fn area(&self) -> u32 {
        self.hight * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.hight > other.hight && self.width > other.width
    }

    fn squre(size: u32) -> Rectangle {
        Rectangle {hight: size, width: size}
    }
}

fn main() {
    println!("This code calculates area of a rectangle");

    let rect1 = Rectangle{ hight: 10, width: 5 };
    let squr1 = Rectangle::squre(3);
    let rect2 = Rectangle{ hight: 8, width: 4 };
    let rect3 = Rectangle{ hight: 11, width: 6 };

    println!("This is the area of the first rectangle{}", rect1.area());
    println!("This is the area of the first Squre {}", squr1.area());
    println!("This is the area of the can hold 2nd rectangle{}", rect1.can_hold(&rect2));
    println!("This is the area of the can hold 3rd rectangle{}", rect1.can_hold(&rect3));
    println!("This is the area of the first rectangle{}", rect1.area());
}
