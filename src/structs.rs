struct Rectangle {
    width: u16,
    height: u16
}
impl Rectangle {
    fn calc(&self, rect: &Rectangle) -> u16 {
        rect.width * rect.height
    }
}

pub fn structs() {
    let rect1: Rectangle = Rectangle { width: 30, height: 50 };

    println!("{}", rect1.calc(&rect1));
}