struct Rectangle {
    width: u16,
    height: u16
}

pub fn structs() {
    let rect1: Rectangle = Rectangle { width: 30, height: 50 };

    println!("{}", calc(&rect1));
}

fn calc(rect: &Rectangle) -> u16 {
    rect.width * rect.height
}