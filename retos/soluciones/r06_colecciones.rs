// =============================================================================
// RETOS 06 — COLECCIONES                         (repasa: 06_colecciones)
// Comprueba tu solución:   cargo test -p retos --test r06
// =============================================================================
// Sustituye cada `todo!()` por tu código. NO modifiques los tests.

use std::collections::{BTreeMap, HashMap, HashSet};

/// RETO 1 — Frecuencia de palabras
/// Cuenta cuántas veces aparece cada palabra. Las palabras se separan por
/// espacios; se pasan a minúsculas y se les quitan los signos de los extremos
/// (todo lo que no sea alfanumérico). Las que quedan vacías se ignoran.
///   "¡Hola, hola mundo!" → {"hola": 2, "mundo": 1}
/// Pista: `trim_matches(|c: char| !c.is_alphanumeric())` y la entry API.
fn frecuencia_palabras(texto: &str) -> HashMap<String, usize> {
    let mut frecuencias = HashMap::new();
    for palabra in texto.split_whitespace() {
        let limpia = palabra
            .trim_matches(|c: char| !c.is_alphanumeric())
            .to_lowercase();
        if !limpia.is_empty() {
            *frecuencias.entry(limpia).or_insert(0) += 1;
        }
    }
    frecuencias
}

#[test]
fn reto_1_frecuencia_palabras() {
    let f = frecuencia_palabras("¡Hola, hola mundo! El MUNDO es... grande -- ");
    assert_eq!(f.len(), 5);
    assert_eq!(f["hola"], 2);
    assert_eq!(f["mundo"], 2);
    assert_eq!(f["grande"], 1);
    assert!(!f.contains_key("--"));
    assert!(frecuencia_palabras("").is_empty());
}

/// RETO 2 — Agrupar por longitud
/// Agrupa las palabras por su número de caracteres, conservando el orden en
/// que aparecen. Un BTreeMap deja las claves ordenadas.
fn agrupar_por_longitud(palabras: &[&str]) -> BTreeMap<usize, Vec<String>> {
    let mut grupos: BTreeMap<usize, Vec<String>> = BTreeMap::new();
    for p in palabras {
        grupos
            .entry(p.chars().count())
            .or_default()
            .push(p.to_string());
    }
    grupos
}

#[test]
fn reto_2_agrupar_por_longitud() {
    let g = agrupar_por_longitud(&["sol", "luna", "mar", "año", "cielo", "nube"]);
    let esperado: Vec<(usize, Vec<&str>)> = vec![
        (3, vec!["sol", "mar", "año"]),
        (4, vec!["luna", "nube"]),
        (5, vec!["cielo"]),
    ];
    let obtenido: Vec<(usize, Vec<&str>)> = g
        .iter()
        .map(|(k, v)| (*k, v.iter().map(String::as_str).collect()))
        .collect();
    assert_eq!(obtenido, esperado);
}

/// RETO 3 — Elementos comunes
/// Devuelve los valores presentes en ambos slices, sin repetir y ordenados.
/// Pista: HashSet e `intersection`.
fn comunes(a: &[i32], b: &[i32]) -> Vec<i32> {
    let a: HashSet<i32> = a.iter().copied().collect();
    let b: HashSet<i32> = b.iter().copied().collect();
    let mut r: Vec<i32> = a.intersection(&b).copied().collect();
    r.sort();
    r
}

#[test]
fn reto_3_comunes() {
    assert_eq!(comunes(&[5, 1, 3, 3, 9], &[3, 9, 9, 2, 5]), [3, 5, 9]);
    assert_eq!(comunes(&[1, 2], &[3, 4]), Vec::<i32>::new());
}

/// RETO 4 — Ranking
/// Devuelve los nombres de los `n` mejores, de mayor a menor puntuación.
/// En caso de empate, por orden alfabético. Si hay menos de `n`, todos.
fn top_n(puntuaciones: &[(&str, u32)], n: usize) -> Vec<String> {
    let mut ordenadas = puntuaciones.to_vec();
    ordenadas.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    ordenadas
        .into_iter()
        .take(n)
        .map(|(nombre, _)| nombre.to_string())
        .collect()
}

#[test]
fn reto_4_top_n() {
    let datos = [
        ("eva", 80),
        ("ana", 95),
        ("luis", 80),
        ("zoe", 60),
        ("bea", 95),
    ];
    assert_eq!(top_n(&datos, 3), ["ana", "bea", "eva"]);
    assert_eq!(top_n(&datos, 10).len(), 5);
    assert!(top_n(&datos, 0).is_empty());
}

/// RETO 5 — Quitar duplicados conservando el orden
/// [3, 1, 3, 2, 1] → [3, 1, 2]
fn sin_duplicados(v: &[i32]) -> Vec<i32> {
    let mut vistos = HashSet::new();
    v.iter().copied().filter(|x| vistos.insert(*x)).collect()
}

#[test]
fn reto_5_sin_duplicados() {
    assert_eq!(sin_duplicados(&[3, 1, 3, 2, 1]), [3, 1, 2]);
    assert_eq!(sin_duplicados(&[]), Vec::<i32>::new());
    assert_eq!(sin_duplicados(&[7, 7, 7]), [7]);
}

/// RETO 6 — Invertir un diccionario
/// De {nombre → ciudad} a {ciudad → nombres ordenados alfabéticamente}.
fn por_ciudad(personas: &HashMap<String, String>) -> BTreeMap<String, Vec<String>> {
    let mut r: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (nombre, ciudad) in personas {
        r.entry(ciudad.clone()).or_default().push(nombre.clone());
    }
    for nombres in r.values_mut() {
        nombres.sort();
    }
    r
}

#[test]
fn reto_6_por_ciudad() {
    let personas: HashMap<String, String> = [
        ("Luis", "Madrid"),
        ("Ana", "Lima"),
        ("Eva", "Madrid"),
        ("Bea", "Madrid"),
    ]
    .into_iter()
    .map(|(n, c)| (n.to_string(), c.to_string()))
    .collect();
    let r = por_ciudad(&personas);
    assert_eq!(r.keys().collect::<Vec<_>>(), ["Lima", "Madrid"]);
    assert_eq!(r["Madrid"], ["Bea", "Eva", "Luis"]);
}
