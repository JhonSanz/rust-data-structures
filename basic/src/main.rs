use std::f64;
mod pointers; // funciona
use pointers::pointers_tour; // no funciona


//
// 2. Funciones
//
fn add(a: i32, b: i32) -> i32 {
    a + b
}

//
// 3. Structs y métodos
//
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn distance_to_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

//
// 4. Enums y match
//
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn move_player(dir: Direction) {
    match dir {
        Direction::Up => println!("Mover arriba"),
        Direction::Down => println!("Mover abajo"),
        Direction::Left => println!("Mover izquierda"),
        Direction::Right => println!("Mover derecha"),
    }
}


//
// 6. Unsafe y punteros crudos
//
fn unsafe_demo() {
    let mut x = 42;
    let ptr: *mut i32 = &mut x;

    unsafe {
        *ptr += 1;
        println!("x modificado por puntero crudo = {}", *ptr);
    }
}

//
// MAIN
//
fn main() {
    pointers_tour();
    // println!("\n--- 2) Functions ---");
    // println!("5 + 3 = {}", add(5, 3));

    // println!("\n--- 3) Structs ---");
    // let p = Point::new(3.0, 4.0);
    // println!("Distancia al origen: {}", p.distance_to_origin());

    // println!("\n--- 4) Enums ---");
    // move_player(Direction::Left);

    // println!("\n--- 6) Unsafe ---");
    // unsafe_demo();
}
