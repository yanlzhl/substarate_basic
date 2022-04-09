use std::f32::consts::PI;

trait Graph{
    fn area(&self) -> f32;
}

impl Graph for Cicle {
    fn area(&self) -> f32 {
         self.radius.powf(self.radius) * PI
    }
}

struct Cicle{
    radius: f32
}

impl Graph for Triangle {
    fn area(&self) -> f32 {
        self.width * self.height / 2.0
    }
}
struct  Triangle{
    width:f32,
    height: f32
}

impl Graph for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }
}

struct Rectangle{
    width:f32,
    height: f32
}

fn print_area<T:Graph> (t:&T){
    println!("{}",t.area())
}



fn main() {
    let cicle = Cicle  { radius: 5.1 };
    let rectangle = Rectangle { width: 3.1, height: 4.2 };
    let triangle = Triangle  { width: 5.3, height: 6.4};

    print_area(&cicle);
    print_area(&rectangle);
    print_area(&triangle);
}