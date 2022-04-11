pub struct Circle {
    r: f64  // 半径
}

pub struct Triangle {
    g: f64, // 底
    h: f64  // 高
}

pub struct Square {
    a: f64  // 边长
}

pub trait Area {
    fn area(&self) -> f64;
}

impl Area for Circle {
    fn area(&self) -> f64 {
        self.r * self.r * std::f64::consts::PI
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        self.g * self.h * 0.5
    }
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.a * self.a
    }
}

pub fn print_area<T: Area>(item: &T) {
    println!("{}", item.area());
}

fn main() {
    let circle = Circle { r: 2.0 };
    let triangle = Triangle { g: 3.0, h: 5.0 };
    let square = Square { a: 2.0 };
    print_area(&circle);
    print_area(&triangle);
    print_area(&square);
}
