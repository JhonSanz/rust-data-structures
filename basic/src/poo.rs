// main.rs

// ---------- Definición de Structs ----------
struct Point {
    x: f64,
    y: f64,
}

struct Circle {
    center: Point,
    radius: f64,
}

struct Square {
    top_left: Point,
    side: f64,
}

// ---------- Métodos con impl ----------
impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn distance_to_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl Circle {
    fn new(x: f64, y: f64, radius: f64) -> Self {
        Self {
            center: Point::new(x, y),
            radius,
        }
    }
}

impl Square {
    fn new(x: f64, y: f64, side: f64) -> Self {
        Self {
            top_left: Point::new(x, y),
            side,
        }
    }
}

// ---------- Traits (Interfaces) ----------
trait Drawable {
    fn draw(&self);
}

// Implementación del trait para Circle
impl Drawable for Circle {
    fn draw(&self) {
        println!(
            "Dibujando un círculo en ({}, {}) con radio {}",
            self.center.x, self.center.y, self.radius
        );
    }
}

// Implementación del trait para Square
impl Drawable for Square {
    fn draw(&self) {
        println!(
            "Dibujando un cuadrado en ({}, {}) con lado {}",
            self.top_left.x, self.top_left.y, self.side
        );
    }
}

// ---------- Funciones polimórficas ----------

// Usando trait objects (dinámico)
fn render(shape: &dyn Drawable) {
    shape.draw();
}

// Usando genéricos (estático)
fn render_all<T: Drawable>(shapes: Vec<T>) {
    for s in shapes {
        s.draw();
    }
}

// ---------- Main ----------
fn main() {
    let p = Point::new(3.0, 4.0);
    println!("Distancia de p al origen: {}", p.distance_to_origin());

    let c = Circle::new(0.0, 0.0, 5.0);
    let s = Square::new(1.0, 1.0, 3.0);

    // Polimorfismo con trait objects
    println!("\n--- Render individual ---");
    render(&c);
    render(&s);

    // Polimorfismo con genéricos
    println!("\n--- Render en batch ---");
    let shapes = vec![Circle::new(2.0, 2.0, 1.5), Circle::new(-1.0, -1.0, 2.0)];
    render_all(shapes);
}
