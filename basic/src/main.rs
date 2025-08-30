use std::f64;

//
// 1. Variables y punteros básicos
//
fn pointers_demo() {
    let x = 42; // stack
    let y = &x; // referencia (puntero seguro)
    let z = *y; // desreferencia

    println!("x = {}, y = {:p}, z = {}", x, y, z);

    let mut a = 10;
    let b = &mut a; // referencia mutable
    *b += 5; // modificar a través del puntero
    println!("a = {}", a);

    /*
    Box se utiliza para almacenar datos en el heap, permitiendo un tamaño dinámico.
    El contenido de un Box se puede acceder de manera similar a una referencia.
    */
    let bx = Box::new(99);
    println!("Box contiene {} {:p}", bx, bx);
}

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
// 5. Funciones built-in
//
fn builtin_demo() {
    let s = String::from("Hola Rust");
    println!("len = {}", s.len());
    println!("is_empty = {}", s.is_empty());
    println!("contains 'Rust'? {}", s.contains("Rust"));
    println!("replace: {}", s.replace("Rust", "Mundo"));

    let nums = vec![1, 2, 3, 4, 5];
    println!("len = {}", nums.len());
    println!("first = {:?}", nums.first());
    println!("last = {:?}", nums.last());
    println!("iter sum = {}", nums.iter().sum::<i32>());
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
    println!("--- 1) Pointers ---");
    pointers_demo();

    // println!("\n--- 2) Functions ---");
    // println!("5 + 3 = {}", add(5, 3));

    // println!("\n--- 3) Structs ---");
    // let p = Point::new(3.0, 4.0);
    // println!("Distancia al origen: {}", p.distance_to_origin());

    // println!("\n--- 4) Enums ---");
    // move_player(Direction::Left);

    // println!("\n--- 5) Builtins ---");
    // builtin_demo();

    // println!("\n--- 6) Unsafe ---");
    // unsafe_demo();
}
