// =============================================================================
// MÓDULO 09 — GENÉRICOS Y LIFETIMES
// Ejecuta con:  cargo run -p genericos
// =============================================================================
//
// GENÉRICOS: código que funciona con muchos tipos. Rust los MONOMORFIZA:
// genera una copia especializada por cada tipo concreto usado, así que son
// tan rápidos como escribir cada versión a mano (pero el binario crece).
//
// LIFETIMES: parámetros genéricos que no hablan de TIPOS sino de cuánto
// TIEMPO vive una referencia. Casi siempre se infieren; solo los escribes
// cuando el compilador no puede deducir la relación entre referencias.

use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::Add;

fn main() {
    println!("===== 1. Funciones genéricas con restricciones =====");
    println!("mayor(&[3, 9, 2]) = {:?}", mayor(&[3, 9, 2]));
    println!(
        "mayor(&[\"pera\", \"uva\"]) = {:?}",
        mayor(&["pera", "uva"])
    );
    println!("mayor::<f64>(&[]) = {:?}", mayor::<f64>(&[]));
    println!("sumar_todo(&[1.5, 2.5]) = {}", sumar_todo(&[1.5, 2.5]));
    println!("sumar_todo(&[1, 2, 3]) = {}", sumar_todo(&[1, 2, 3]));

    println!("\n===== 2. Varias restricciones y where =====");
    mostrar_pares(&[("uno", 1), ("dos", 2)]);
    let conteo = contar(["a", "b", "a", "c", "a"]);
    let mut v: Vec<_> = conteo.into_iter().collect();
    v.sort();
    println!("contar = {v:?}");

    println!("\n===== 3. Structs y enums genéricos =====");
    structs_genericos();

    println!("\n===== 4. Traits con genéricos y tipos asociados =====");
    let mut p = Pila::new();
    p.meter(1);
    p.meter(2);
    println!("contenedor: len={} primero={:?}", p.tamano(), p.primero());

    println!("\n===== 5. Const generics =====");
    let m = Matriz::<2, 3>::identidad_parcial();
    println!(
        "Matriz<2,3> = {:?} (filas={}, cols={})",
        m.datos,
        m.filas(),
        m.columnas()
    );
    println!("suma de array [1,2,3,4] = {}", suma_array([1, 2, 3, 4]));

    println!("\n===== 6. Lifetimes =====");
    lifetimes();
}

// <T: PartialOrd> = "para cualquier tipo T que se pueda comparar con <"
fn mayor<T: PartialOrd + Copy>(lista: &[T]) -> Option<T> {
    let mut iter = lista.iter();
    let mut max = *iter.next()?;
    for &x in iter {
        if x > max {
            max = x;
        }
    }
    Some(max)
}

// Restricción con traits numéricos de std::ops. `Default` da el "cero".
fn sumar_todo<T: Add<Output = T> + Default + Copy>(xs: &[T]) -> T {
    xs.iter().fold(T::default(), |acc, &x| acc + x)
}

// `where` hace legibles las firmas con muchas restricciones
fn mostrar_pares<K, V>(pares: &[(K, V)])
where
    K: Display,
    V: Debug,
{
    for (k, v) in pares {
        println!("  {k} => {v:?}");
    }
}

// Acepta cualquier cosa iterable: arrays, Vec, iteradores...
fn contar<T, I>(items: I) -> HashMap<T, usize>
where
    I: IntoIterator<Item = T>,
    T: Eq + Hash,
{
    let mut m = HashMap::new();
    for x in items {
        *m.entry(x).or_insert(0) += 1;
    }
    m
}

// --- Tipos genéricos ----------------------------------------------------------
#[derive(Debug, Clone, Copy)]
struct Punto<T> {
    x: T,
    y: T,
}

// Métodos para CUALQUIER T
impl<T: Copy> Punto<T> {
    fn intercambiar(self) -> Punto<T> {
        Punto {
            x: self.y,
            y: self.x,
        }
    }
}

// Métodos SOLO para Punto<f64> (especialización por tipo concreto)
impl Punto<f64> {
    fn distancia_al_origen(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

// Option<T> y Result<T, E> son simplemente enums genéricos de la std:
#[derive(Debug)]
#[allow(dead_code)]
enum Arbol<T> {
    Hoja(T),
    Nodo(Box<Arbol<T>>, Box<Arbol<T>>), // Box: un tipo recursivo necesita indirección
}

impl<T: Copy + Add<Output = T>> Arbol<T> {
    fn suma(&self) -> T {
        match self {
            Arbol::Hoja(v) => *v,
            Arbol::Nodo(i, d) => i.suma() + d.suma(),
        }
    }
}

fn structs_genericos() {
    let pi = Punto { x: 1, y: 2 };
    let pf = Punto { x: 3.0, y: 4.0 };
    println!("{pi:?} intercambiado = {:?}", pi.intercambiar());
    println!("{pf:?} distancia = {}", pf.distancia_al_origen());
    // pi.distancia_al_origen(); // ERROR: solo existe para Punto<f64>

    let arbol = Arbol::Nodo(
        Box::new(Arbol::Hoja(1)),
        Box::new(Arbol::Nodo(
            Box::new(Arbol::Hoja(2)),
            Box::new(Arbol::Hoja(3)),
        )),
    );
    println!("suma del árbol = {}", arbol.suma());
}

// --- Trait con tipo asociado (como Iterator::Item) -----------------------------
trait Contenedor {
    type Item; // el implementador decide qué tipo es
    fn tamano(&self) -> usize;
    fn primero(&self) -> Option<&Self::Item>;
}

struct Pila<T> {
    elementos: Vec<T>,
}

impl<T> Pila<T> {
    fn new() -> Self {
        Pila {
            elementos: Vec::new(),
        }
    }
    fn meter(&mut self, x: T) {
        self.elementos.push(x);
    }
}

impl<T> Contenedor for Pila<T> {
    type Item = T;
    fn tamano(&self) -> usize {
        self.elementos.len()
    }
    fn primero(&self) -> Option<&T> {
        self.elementos.first()
    }
}

// --- Const generics: parámetros que son VALORES conocidos en compilación ------
struct Matriz<const F: usize, const C: usize> {
    datos: [[i32; C]; F],
}

impl<const F: usize, const C: usize> Matriz<F, C> {
    fn identidad_parcial() -> Self {
        let mut datos = [[0; C]; F];
        for (i, fila) in datos.iter_mut().enumerate().take(F.min(C)) {
            fila[i] = 1;
        }
        Matriz { datos }
    }
    fn filas(&self) -> usize {
        F
    }
    fn columnas(&self) -> usize {
        C
    }
}

fn suma_array<const N: usize>(arr: [i32; N]) -> i32 {
    arr.iter().sum()
}

// --- Lifetimes ----------------------------------------------------------------
// ¿Cuánto vive la referencia devuelta? Puede venir de `a` o de `b`, así que el
// compilador necesita que se lo digamos: 'a significa "el resultado vive como
// mucho tanto como la MÁS CORTA de las dos entradas".
fn mas_larga<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

// Elisión de lifetimes: con UNA referencia de entrada no hace falta escribirlo.
// Esta firma equivale a `fn primera_palabra<'a>(s: &'a str) -> &'a str`.
fn primera_palabra(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

// Un struct que GUARDA una referencia necesita un lifetime: el struct no
// puede vivir más que el texto que referencia.
#[derive(Debug)]
struct Extracto<'a> {
    parte: &'a str,
}

impl<'a> Extracto<'a> {
    // Por la elisión, &self → el resultado vive como self... pero aquí
    // devolvemos algo con el lifetime del TEXTO ORIGINAL ('a), que es mejor.
    fn parte(&self) -> &'a str {
        self.parte
    }
}

fn lifetimes() {
    let s1 = String::from("una cadena larga");
    let resultado;
    {
        let s2 = String::from("corta");
        resultado = mas_larga(&s1, &s2);
        println!("mas_larga = {resultado:?}");
    }
    // println!("{resultado}"); // ERROR: s2 ya murió y `resultado` podría apuntar a ella

    println!("primera_palabra = {:?}", primera_palabra(&s1));

    let novela = String::from("Llamadme Ismael. Hace unos años...");
    let primera_frase = novela.split('.').next().unwrap();
    let e = Extracto {
        parte: primera_frase,
    };
    println!("{e:?} → parte() = {:?}", e.parte());

    // 'static: vive todo el programa (literales, constantes, datos "filtrados")
    let fijo: &'static str = "vivo para siempre";
    println!("'static: {fijo}");

    // Consejo: si luchas mucho con los lifetimes en un struct, a menudo es
    // mejor que el struct sea DUEÑO de sus datos (String en vez de &str).
}
