pub struct Rect {
    pub height: f32,
    pub width: f32,
}

impl Rect {
    pub fn area(&self) -> f32 {
        return self.height * self.width;
    }
    pub fn print_something(){
        println!("This is a rectangle");
    }
}