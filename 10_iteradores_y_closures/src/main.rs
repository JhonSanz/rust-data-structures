// =============================================================================
// MÓDULO 10 — ITERADORES
// Ejecuta con:  cargo run -p iteradores
// =============================================================================
//
// Un iterador es cualquier tipo que implementa el trait Iterator:
//
//   trait Iterator {
//       type Item;
//       fn next(&mut self) -> Option<Self::Item>;
//       // ...y más de 70 métodos con implementación por defecto
//   }
//
// Los iteradores son PEREZOSOS: los adaptadores (map, filter...) no hacen
// nada hasta que un consumidor (collect, sum, for...) pide elementos.
// Y son de COSTE CERO: el compilador los convierte en bucles tan rápidos
// como los escritos a mano (a menudo más, porque evita comprobar índices).

use std::collections::HashMap;

fn main() {
    println!("===== 1. iter, iter_mut, into_iter =====");
    tres_formas();

    println!("\n===== 2. Adaptadores (perezosos) =====");
    adaptadores();

    println!("\n===== 3. Consumidores =====");
    consumidores();

    println!("\n===== 4. Pereza: nada ocurre sin consumidor =====");
    pereza();

    println!("\n===== 5. Iterador propio =====");
    let fib: Vec<u64> = Fibonacci::new().take_while(|&x| x < 100).collect();
    println!("fibonacci < 100: {fib:?}");
    let cuenta_atras: Vec<_> = CuentaAtras(5).collect();
    println!("cuenta atrás: {cuenta_atras:?}");

    println!("\n===== 6. Generadores rápidos: from_fn, successors, repeat =====");
    generadores();

    println!("\n===== 7. Ejemplo real: procesar un CSV =====");
    procesar_ventas();
}

fn tres_formas() {
    let mut v = vec![String::from("a"), String::from("b")];

    // .iter()      → &T      (préstamo de lectura)
    for s in v.iter() {
        print!("{s} ");
    }
    // .iter_mut()  → &mut T  (préstamo mutable)
    for s in v.iter_mut() {
        s.push('!');
    }
    // .into_iter() → T       (consume la colección)
    let consumido: Vec<String> = v.into_iter().map(|s| s.to_uppercase()).collect();
    println!("→ {consumido:?}");
    // v ya no existe aquí

    // `for x in &v` es azúcar para `for x in v.iter()`, y `for x in v` para into_iter().
}

fn adaptadores() {
    let n = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let r: Vec<i32> = n.iter().map(|x| x * x).collect();
    println!("map(x²):        {r:?}");
    let r: Vec<&i32> = n.iter().filter(|&&x| x % 2 == 0).collect();
    println!("filter(par):    {r:?}");
    let r: Vec<i32> = n
        .iter()
        .filter_map(|&x| if x % 3 == 0 { Some(x * 10) } else { None })
        .collect();
    println!("filter_map:     {r:?}");
    let r: Vec<_> = n.iter().skip(2).step_by(3).collect();
    println!("skip+step_by:   {r:?}");
    let r: Vec<_> = n.iter().take_while(|&&x| x < 4).collect();
    println!("take_while(<4): {r:?}");
    let r: Vec<_> = n.iter().rev().take(3).collect();
    println!("rev+take(3):    {r:?}");

    let letras = ['a', 'b', 'c'];
    let r: Vec<_> = letras.iter().zip(n.iter()).collect();
    println!("zip:            {r:?}");
    let r: Vec<_> = letras.iter().enumerate().collect();
    println!("enumerate:      {r:?}");
    let r: Vec<_> = letras.iter().chain(['x', 'y'].iter()).collect();
    println!("chain:          {r:?}");

    let frases = ["hola mundo", "adiós amigo"];
    let r: Vec<&str> = frases.iter().flat_map(|f| f.split(' ')).collect();
    println!("flat_map:       {r:?}");
    let r: Vec<i32> = vec![vec![1, 2], vec![3]].into_iter().flatten().collect();
    println!("flatten:        {r:?}");

    // scan: como fold pero emitiendo cada estado intermedio (suma acumulada)
    let r: Vec<i32> = n
        .iter()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect();
    println!("scan(suma acc): {r:?}");

    // peekable: mirar el siguiente sin consumirlo
    let mut it = n.iter().peekable();
    while let Some(x) = it.next() {
        if let Some(&&sig) = it.peek()
            && sig == 5
        {
            println!("peekable: el siguiente a {x} es 5");
        }
    }
}

fn consumidores() {
    let n = [3, 1, 4, 1, 5, 9, 2, 6];

    println!(
        "sum = {}, product = {}",
        n.iter().sum::<i32>(),
        n.iter().product::<i32>()
    );
    println!(
        "count = {}, min = {:?}, max = {:?}",
        n.len(),
        n.iter().min(),
        n.iter().max()
    );
    println!(
        "fold (concatenar) = {}",
        n.iter().fold(String::new(), |acc, x| acc + &x.to_string())
    );
    println!("reduce (máximo) = {:?}", n.iter().copied().reduce(i32::max));
    println!(
        "any(>8) = {}, all(>0) = {}",
        n.iter().any(|&x| x > 8),
        n.iter().all(|&x| x > 0)
    );
    println!(
        "find(>4) = {:?}, position(==9) = {:?}",
        n.iter().find(|&&x| x > 4),
        n.iter().position(|&x| x == 9)
    );
    println!(
        "last = {:?}, rev().nth(2) = {:?}",
        n.iter().last(),
        n.iter().rev().nth(2)
    );
    println!("max_by_key(-x) = {:?}", n.iter().max_by_key(|&&x| -x));

    // partition: dividir en dos colecciones
    let (pares, impares): (Vec<i32>, Vec<i32>) = n.iter().partition(|&&x| x % 2 == 0);
    println!("partition: pares {pares:?}, impares {impares:?}");

    // collect sabe construir muchos tipos: lo decide la anotación
    let texto: String = vec!['h', 'o', 'l', 'a'].into_iter().collect();
    let mapa: HashMap<&str, usize> = ["uno", "tres"].iter().map(|s| (*s, s.len())).collect();
    let unicos: std::collections::BTreeSet<_> = n.iter().collect();
    println!(
        "collect → String {texto:?}, HashMap con {} claves, BTreeSet {unicos:?}",
        mapa.len()
    );

    // unzip
    let (nombres, edades): (Vec<&str>, Vec<u32>) = [("Ana", 30), ("Luis", 25)].into_iter().unzip();
    println!("unzip: {nombres:?} {edades:?}");

    // for_each para efectos secundarios (aunque un for suele ser más claro)
    n.iter().take(3).for_each(|x| print!("[{x}]"));
    println!();
}

fn pereza() {
    let it = (1..=3).map(|x| {
        println!("  procesando {x}");
        x * 2
    });
    println!("iterador creado, pero aún no se ha procesado nada");
    let total: i32 = it.sum();
    println!("total = {total}");

    // Por eso los iteradores infinitos son posibles:
    let primer_cuadrado_mayor_de_500 = (1..).map(|x| x * x).find(|&x| x > 500);
    println!("primer cuadrado > 500: {primer_cuadrado_mayor_de_500:?}");
}

// --- Iteradores propios -------------------------------------------------------
struct Fibonacci {
    actual: u64,
    siguiente: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci {
            actual: 0,
            siguiente: 1,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;
    // Implementar solo `next` nos da GRATIS map, filter, take, sum, zip...
    fn next(&mut self) -> Option<u64> {
        let r = self.actual;
        self.actual = self.siguiente;
        self.siguiente += r;
        Some(r) // nunca devuelve None: es infinito
    }
}

struct CuentaAtras(u32);

impl Iterator for CuentaAtras {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.0 == 0 {
            None
        } else {
            self.0 -= 1;
            Some(self.0 + 1)
        }
    }
}

fn generadores() {
    // from_fn: un iterador a partir de un closure con estado
    let mut n = 1;
    let potencias: Vec<u32> = std::iter::from_fn(|| {
        n *= 2;
        (n <= 64).then_some(n)
    })
    .collect();
    println!("from_fn (potencias de 2): {potencias:?}");

    // successors: cada elemento se calcula a partir del anterior
    let collatz: Vec<u64> = std::iter::successors(Some(6u64), |&x| {
        if x == 1 {
            None
        } else if x % 2 == 0 {
            Some(x / 2)
        } else {
            Some(3 * x + 1)
        }
    })
    .collect();
    println!("successors (Collatz de 6): {collatz:?}");

    let guiones: String = std::iter::repeat_n('-', 10).collect();
    println!("repeat_n: {guiones}");
}

fn procesar_ventas() {
    let csv = "\
producto,categoria,unidades,precio
manzana,fruta,10,0.5
pan,panadería,3,1.2
pera,fruta,4,0.7
croissant,panadería,6,1.5
leche,lácteos,2,0.9
linea,rota
";
    let mut por_categoria: HashMap<&str, f64> = HashMap::new();
    let mut errores = 0;

    for linea in csv.lines().skip(1).filter(|l| !l.trim().is_empty()) {
        let campos: Vec<&str> = linea.split(',').collect();
        let [_, categoria, unidades, precio] = campos[..] else {
            errores += 1;
            continue;
        };
        let (Ok(u), Ok(p)) = (unidades.parse::<f64>(), precio.parse::<f64>()) else {
            errores += 1;
            continue;
        };
        *por_categoria.entry(categoria).or_default() += u * p;
    }

    let mut ranking: Vec<(&str, f64)> = por_categoria.into_iter().collect();
    ranking.sort_by(|a, b| b.1.total_cmp(&a.1));
    for (i, (cat, total)) in ranking.iter().enumerate() {
        println!("  {}. {cat:<10} {total:>6.2} €", i + 1);
    }
    println!("  líneas con error: {errores}");
}
