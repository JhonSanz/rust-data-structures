// =============================================================================
// MÓDULO 06 — COLECCIONES
// Ejecuta con:  cargo run -p colecciones
// =============================================================================
//
// | Tipo            | Qué es                                   | Equivalente en Go     |
// |-----------------|------------------------------------------|-----------------------|
// | [T; N]          | array de tamaño fijo (en la pila)         | [N]T                  |
// | Vec<T>          | vector que crece (en el heap)             | []T                   |
// | &[T] / &mut [T] | slice: vista prestada a una secuencia     | []T (sub-slice)       |
// | String / &str   | texto UTF-8 propio / prestado             | string                |
// | HashMap<K, V>   | diccionario sin orden                     | map[K]V               |
// | BTreeMap<K, V>  | diccionario ORDENADO por clave            | —                     |
// | HashSet<T>      | conjunto sin orden                        | map[T]struct{}        |
// | VecDeque<T>     | cola de doble extremo                     | —                     |
// | BinaryHeap<T>   | montículo (cola de prioridad)             | container/heap        |

use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    println!("===== 1. Vec<T> =====");
    vectores();

    println!("\n===== 2. Slices y métodos útiles =====");
    slices();

    println!("\n===== 3. String como colección =====");
    strings();

    println!("\n===== 4. HashMap =====");
    hashmaps();

    println!("\n===== 5. La entry API =====");
    entry_api();

    println!("\n===== 6. BTreeMap y HashSet =====");
    btree_y_sets();

    println!("\n===== 7. VecDeque y BinaryHeap =====");
    colas();
}

fn vectores() {
    // Crear
    let mut v: Vec<i32> = Vec::new();
    let w = vec![1, 2, 3]; // macro vec!
    let ceros = vec![0; 5];
    let con_capacidad: Vec<String> = Vec::with_capacity(100);
    println!(
        "w={w:?} ceros={ceros:?} capacidad reservada={}",
        con_capacidad.capacity()
    );

    // Añadir y quitar
    v.push(10);
    v.push(20);
    v.push(30);
    v.extend([40, 50]);
    v.insert(0, 5);
    let ultimo = v.pop(); // Option<i32>
    let quitado = v.remove(1); // por índice (desplaza el resto)
    println!("v={v:?} pop={ultimo:?} remove(1)={quitado}");

    // Crecimiento: len vs capacity (como en Go, pero sin sorpresas de aliasing:
    // el borrow checker impide tener una referencia mientras el Vec se realoja)
    let mut crece = Vec::new();
    for i in 0..10 {
        crece.push(i);
        print!("({},{}) ", crece.len(), crece.capacity());
    }
    println!();

    // Acceso
    println!(
        "v[0]={} v.get(99)={:?} first={:?} last={:?}",
        v[0],
        v.get(99),
        v.first(),
        v.last()
    );
    // v[99]; // PANIC en ejecución: index out of bounds (nunca memoria corrupta)

    // Recorrer
    for x in &v {
        print!("{x} "); // &v → referencias: v sigue siendo nuestra
    }
    println!();
    for x in &mut v {
        *x *= 2; // `*` desreferencia para modificar el valor apuntado
    }
    println!("doblado: {v:?}");

    // retain: filtrar EN SITIO
    v.retain(|&x| x > 30);
    println!("retain(>30): {v:?}");

    // Vec 2D
    let mut matriz = vec![vec![0; 3]; 2];
    matriz[1][2] = 7;
    println!("matriz = {matriz:?}");
}

fn slices() {
    let mut datos = vec![5, 3, 8, 1, 9, 2, 7];

    // Subslices por rango
    println!(
        "&datos[2..5] = {:?}, &datos[..2] = {:?}",
        &datos[2..5],
        &datos[..2]
    );

    // Ordenar y buscar
    datos.sort();
    println!("sort: {datos:?}");
    println!("binary_search(&8) = {:?}", datos.binary_search(&8));
    datos.sort_by(|a, b| b.cmp(a)); // descendente
    println!("sort desc: {datos:?}");
    println!(
        "contains(&9)={} position(1)={:?}",
        datos.contains(&9),
        datos.iter().position(|&x| x == 1)
    );

    // Máximo, mínimo, suma (vía iteradores, módulo 10)
    println!(
        "max={:?} min={:?} sum={}",
        datos.iter().max(),
        datos.iter().min(),
        datos.iter().sum::<i32>()
    );

    // Trocear
    for trozo in datos.chunks(3) {
        print!("{trozo:?} ");
    }
    println!("← chunks(3)");
    for ventana in datos.windows(2).take(3) {
        print!("{ventana:?} ");
    }
    println!("← windows(2)");

    let (izq, der) = datos.split_at(3);
    println!("split_at(3): {izq:?} | {der:?}");

    // Otras: dedup, reverse, swap, concat, join
    let mut repes = vec![1, 1, 2, 2, 2, 3, 1];
    repes.dedup(); // quita repetidos CONSECUTIVOS
    repes.reverse();
    println!("dedup+reverse: {repes:?}");
    let partes = [vec![1, 2], vec![3], vec![4, 5]];
    println!("concat: {:?}", partes.concat());
    println!("join: {}", ["a", "b", "c"].join("-"));
}

fn strings() {
    let mut s = String::from("Hola");
    s.push_str(" mundo");
    s.insert(0, '¡');
    s += "!";
    println!("{s}");

    println!("mayúsculas: {}", s.to_uppercase());
    println!("contains(\"mundo\"): {}", s.contains("mundo"));
    println!("replace: {}", s.replace("mundo", "Rust"));
    println!("split: {:?}", "a,b,,c".split(',').collect::<Vec<_>>());
    println!(
        "split_whitespace: {:?}",
        "  uno  dos tres ".split_whitespace().collect::<Vec<_>>()
    );
    println!("trim: {:?}", "  hola  ".trim());
    println!("starts_with: {}", s.starts_with('¡'));
    println!("split_once: {:?}", "clave=valor".split_once('='));
    println!("lines: {:?}", "l1\nl2\nl3".lines().collect::<Vec<_>>());
    println!("repeat: {}", "ab".repeat(3));
    println!(
        "chars().rev(): {}",
        "Rust".chars().rev().collect::<String>()
    );
}

fn hashmaps() {
    let mut edades: HashMap<String, u32> = HashMap::new();
    edades.insert("Ana".to_string(), 30);
    edades.insert("Luis".to_string(), 25);
    edades.insert("Eva".to_string(), 35);

    // insert devuelve el valor anterior si existía
    let anterior = edades.insert("Ana".to_string(), 31);
    println!("anterior de Ana: {anterior:?}");

    // Leer: get devuelve Option<&V> (no hay "valor cero" como en Go)
    match edades.get("Luis") {
        Some(e) => println!("Luis tiene {e}"),
        None => println!("Luis no está"),
    }
    println!("get(\"Zoe\") = {:?}", edades.get("Zoe"));
    println!("contains_key(\"Eva\") = {}", edades.contains_key("Eva"));
    // edades["Zoe"]; // PANIC si no existe

    // Modificar un valor existente
    if let Some(e) = edades.get_mut("Eva") {
        *e += 1;
    }
    edades.remove("Luis");

    // El orden de iteración NO está definido (y cambia entre ejecuciones).
    // Para un orden estable, ordena las claves o usa BTreeMap.
    let mut claves: Vec<_> = edades.keys().collect();
    claves.sort();
    for k in claves {
        println!("  {k}: {}", edades[k]);
    }

    // Construir un HashMap desde un iterador de tuplas
    let precios: HashMap<&str, f64> = [("pan", 1.2), ("leche", 0.9)].into_iter().collect();
    println!("precios tiene {} elementos", precios.len());
}

fn entry_api() {
    // entry() es LA forma idiomática de "insertar o actualizar" con una sola búsqueda.
    let texto = "el gato y el perro y el pez";
    let mut frecuencias: HashMap<&str, usize> = HashMap::new();
    for palabra in texto.split_whitespace() {
        *frecuencias.entry(palabra).or_insert(0) += 1;
    }
    let mut ordenadas: Vec<_> = frecuencias.into_iter().collect();
    ordenadas.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    println!("frecuencias: {ordenadas:?}");

    // Agrupar: or_default() crea el valor por defecto (aquí, un Vec vacío)
    let animales = ["gato", "perro", "gallina", "pez", "pato"];
    let mut por_inicial: BTreeMap<char, Vec<&str>> = BTreeMap::new();
    for a in animales {
        por_inicial
            .entry(a.chars().next().unwrap())
            .or_default()
            .push(a);
    }
    println!("agrupados: {por_inicial:?}");
}

fn btree_y_sets() {
    // BTreeMap: siempre ordenado por clave, permite rangos
    let mut notas = BTreeMap::new();
    notas.insert(85, "Ana");
    notas.insert(92, "Eva");
    notas.insert(78, "Luis");
    notas.insert(60, "Zoe");
    println!("ordenado: {notas:?}");
    println!("notas >= 80: {:?}", notas.range(80..).collect::<Vec<_>>());
    println!(
        "primera: {:?}, última: {:?}",
        notas.first_key_value(),
        notas.last_key_value()
    );

    // HashSet: elementos únicos, operaciones de conjuntos
    let a: HashSet<i32> = [1, 2, 3, 4].into();
    let b: HashSet<i32> = [3, 4, 5].into();
    let ordenar = |s: HashSet<&i32>| {
        let mut v: Vec<_> = s.into_iter().copied().collect();
        v.sort();
        v
    };
    println!("unión:        {:?}", ordenar(a.union(&b).collect()));
    println!("intersección: {:?}", ordenar(a.intersection(&b).collect()));
    println!("diferencia:   {:?}", ordenar(a.difference(&b).collect()));
    println!("¿a contiene 2? {}", a.contains(&2));

    // Quitar duplicados conservando el orden de aparición
    let mut vistos = HashSet::new();
    let unicos: Vec<_> = [3, 1, 3, 2, 1]
        .into_iter()
        .filter(|x| vistos.insert(*x))
        .collect();
    println!("únicos en orden: {unicos:?}");
}

fn colas() {
    // VecDeque: push/pop eficientes por ambos extremos (FIFO, ventanas...)
    let mut cola = VecDeque::new();
    cola.push_back("tarea 1");
    cola.push_back("tarea 2");
    cola.push_front("urgente");
    while let Some(t) = cola.pop_front() {
        print!("{t} → ");
    }
    println!("fin");

    // BinaryHeap: siempre saca el MAYOR. Para el menor, usa std::cmp::Reverse.
    let mut heap = BinaryHeap::from([3, 1, 4, 1, 5, 9, 2, 6]);
    let mut top3 = vec![];
    for _ in 0..3 {
        top3.push(heap.pop().unwrap());
    }
    println!("top 3: {top3:?}");
    let mut minimos = BinaryHeap::new();
    for x in [5, 2, 8] {
        minimos.push(std::cmp::Reverse(x));
    }
    println!("menor: {:?}", minimos.pop().map(|r| r.0));
}
