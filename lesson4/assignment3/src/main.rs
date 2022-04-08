use std::f64::consts::PI;

struct Square {
    length: f64,
}

struct Circular {
    radius: f64,
}

struct Triangle {
    height: f64,
    length: f64,
}

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Square {
    fn area(&self) -> f64 {
        return self.length * self.length;
    }
}

impl Shape for Circular {
    fn area(&self) -> f64 {
        return self.radius * self.radius * PI;
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        return (self.height * self.length) / 2.0;
    }
}

fn print_area<T: Shape>(shape: &T) {
    println!("area: {}", shape.area())
}

fn main() {
    print_area(&Square { length: 10.0 });
    print_area(&Circular { radius: 10.0 });
    print_area(&Triangle {
        length: 10.0,
        height: 10.0,
    });
}
