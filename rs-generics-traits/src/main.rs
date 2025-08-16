use std::f32::consts::PI;

trait Shape {
    fn area(&self) -> f32;
}

struct Circle {
    radius: f32,
}

struct Rect {
    width: f32,
    height: f32,
}

impl Shape for Rect {
    fn area(&self) -> f32 {
        return self.width * self.height;
    }
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        return PI * self.radius * self.radius;
    }
}

fn print_area_of_shape<T: Shape>(s: T) {
    println!("{}", s.area());
}

fn main() {
    let r: Rect = Rect {
        width: 10.0,
        height: 10.0,
    };
    let c: Circle = Circle { radius: 10.0 };
    print_area_of_shape(r);
    print_area_of_shape(c);
}
