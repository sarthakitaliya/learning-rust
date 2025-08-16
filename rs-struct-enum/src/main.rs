use std::f32::consts::PI;

use rect::Rect;
pub mod rect;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
enum Shape {
    Square(f32),
    Circle(f32),
    Rectangle(f32, f32),
}
fn main() {
    let rect = Rect {
        height: 30.0,
        width: 50.0,
    };

    let dir = Direction::Up;
    steer(dir);
    let shape = Shape::Square(10.0);
    let area = calculate_area(shape);
    println!("Area: {}", area);

    println!("Rectangle: {} x {}", rect.width, rect.height);
    println!("Area: {}", rect.area());
    Rect::print_something(); //kind of like a static method
}

fn steer(dir: Direction) {
    match dir {
        Direction::Up => println!("Steering Up"),
        Direction::Down => println!("Steering Down"),
        Direction::Left => println!("Steering Left"),
        Direction::Right => println!("Steering Right"),
        _ => println!("Unknown Direction"),
    }
}

fn calculate_area(shape: Shape) -> f32 {
    match shape {
        Shape::Square(side) => return side * side,
        Shape::Circle(radius) => return PI * radius * radius,
        Shape::Rectangle(width, height) => return width * height,
    }
}
