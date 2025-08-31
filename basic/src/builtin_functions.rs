pub fn vector_demo() {
    /*
    ----------------------------------------
        Metodos basicos
    ----------------------------------------
    */
    println!("---------------------------------------------------------------");
    println!("MÉTODOS BÁSICOS");
    let mut v1 = Vec::new();
    for i in 0..9 {
        v1.push(i * 10);
    }

    println!("{:?}", v1);
    println!("Primer elemento: {} {}", v1[0], v1.first().unwrap());
    println!("Tamaño del vector: {}", v1.len());

    v1.pop();

    println!("Último elemento: {:?}", v1.last().unwrap());
    println!("Tamaño del vector: {}", v1.len());
    println!("¿Está vacío el vector? {}", v1.is_empty());
    println!("Elemento 2: {}", v1.get(2).unwrap());
    println!("---------------------------------------------------------------");

    // inicializar
    println!("INICIALIZAR VECTORES");

    let mut v2 = vec![0; 5]; // [0, 0, 0, 0, 0]
    println!("{:?}", v2); // [0, 0, 0, 0, 0]
    let mut v2: Vec<i32> = (1..6).collect();
    println!("{:?}", v2); // [1, 2, 3, 4, 5]
    println!("---------------------------------------------------------------");


    /*
    ----------------------------------------
        Slicing
    ----------------------------------------

    Hace referencia a una porción del vector sin copiar los datos. 
    Si se desea copiar, se puede hacer con el método to_vec().
    */
    println!("SLICING");

    let mut v3 = vec![10, 20, 30, 40, 50];
    let slice = &v3[1..4]; // NO copia datos
    let slice1 = &v3[..];     // todo el vector
    let slice2 = &v3[..3];    // primeros 3 -> [10,20,30]
    let slice3 = &v3[2..];    // desde índice 2 -> [30,40,50]

    println!("{:?}", slice);
    println!("{:?}", slice1);
    println!("{:?}", slice2);
    println!("{:?}", slice3);

    // Copia de un slice
    let copia: Vec<_> = v3[1..4].to_vec();
    println!("{:?}", copia);

    // Los dos comparten memoria
    println!("Vec original en memoria: {:p}", v3.as_ptr());
    println!("Slice apuntando a:       {:p}", slice.as_ptr());

    // Aqui modificamos un valor del slice, como vemos SI MODIFICA EL ORIGINAL
    let slice4 = &mut v3[1..4]; // slice4 mut → referencia
    slice4[0] = 999;           // modifica v[1]

    println!("Slice4: {:?}", slice4);   // [999, 30, 40]
    println!("Vec original: {:?}", v3); // [10, 999, 30, 40, 50]  <-- ¡sí cambió!
    println!("---------------------------------------------------------------");

    /*
    ----------------------------------------
        Iteradores
    ----------------------------------------
    */

    /*
    Iterar SOLO LECTURA.
    tanto for x in &v4 como for x in v4.iter() son equivalentes.
    */
    println!("ITERADORES");
    let v4 = vec![10, 20, 30, 40, 50];

    for x in &v4 {
         println!("leyendo: {}", x);
    }

    for x in v4.iter() { // Devuelve &T para cada elemento.
        println!("leyendo: {}", x);
    }

    /*
    ----------------------------------------
    Referencias mutables

    Devuelve &mut T para cada elemento.
    Puedes leer y escribir sobre cada elemento.
    ----------------------------------------
    */

    let mut v5 = vec![10, 20, 30];

    for x in v5.iter_mut() {
        *x *= 10; // modifica cada elemento
    }

    println!("modificado: {:?}", v5);
    println!("original: {:?}", v5);


    /*
    ----------------------------------------
    Consumo del vector

    Devuelve T (los valores en sí, no referencias).
    El vector se mueve dentro del iterador → ya no puedes usar v después.
    ----------------------------------------
    */

    let v = vec![10, 20, 30];

    for x in v.into_iter() {
        println!("consumiendo: {}", x); // x es i32
    }

    // println!("{:?}", v); // ❌ ERROR: v ya fue movido
}


pub fn string_demo() {
    /*
    ----------------------------------------
        Métodos básicos
    ----------------------------------------
    */

    // Crear un String vacío
    let mut s1 = String::new();
    s1.push_str("Hola");
    s1.push(' '); // un solo caracter
    s1.push_str("Mundo");

    println!("String: {}", s1);
    println!("Tamaño en bytes: {}", s1.len());
    println!("Capacidad: {}", s1.capacity());
    println!("¿Está vacío?: {}", s1.is_empty());

    // Crear a partir de &str (slice de cadena literal)
    let mut s2 = String::from("Rust");
    let s3 = "Programación".to_string();

    println!("s2: {}", s2);
    println!("s3: {}", s3);

    // Concatenar
    s2.push_str(" 🦀");
    println!("s2 concatenado: {}", s2);

    /*
    ----------------------------------------
        Indexación y slicing
    ----------------------------------------

    ⚠️ String en Rust está codificado como UTF-8 → NO permite indexar por posición directamente.
    Necesitas slices de rango. PERO estos rangos deben coincidir con límites válidos de UTF-8.

    Ejemplo: "¡Hola!" tiene caracteres multibyte.
    */

    let s4 = String::from("¡Hola Rust!");
    let slice = &s4[0..5]; // "¡Hol" → toma bytes 0..5
    println!("Slice: {}", slice);

    // ⚠️ si cortas en medio de un caracter UTF-8 → panic
    // let slice_err = &s4[0..2]; // ❌ puede romper si cae a la mitad de "¡"

    // Copiar un slice
    let copia = s4[0..5].to_string();
    println!("Copia del slice: {}", copia);

    /*
    ----------------------------------------
        Iteradores
    ----------------------------------------

    String se puede iterar en distintos niveles:
    - .chars() → caracteres Unicode (char)
    - .bytes() → bytes individuales (u8)
    */

    let s5 = String::from("Rust 🦀");

    println!("Iterando por chars:");
    for c in s5.chars() {
        println!("{}", c);
    }

    println!("Iterando por bytes:");
    for b in s5.bytes() {
        println!("{}", b);
    }

    /*
    ----------------------------------------
        Mutabilidad
    ----------------------------------------
    */

    let mut s6 = String::from("Hola");
    s6.push('!');
    println!("Mutado: {}", s6);

    /*
    ----------------------------------------
        Consumo del String
    ----------------------------------------

    Al usar into_bytes() o into_iter() puedes consumir el String
    y obtener sus datos.
    */

    let s7 = String::from("ABC");

    for c in s7.into_bytes() {
        println!("byte consumido: {}", c);
    }

    // println!("{}", s7); // ❌ ERROR: ya fue movido

    /*
    ----------------------------------------
        Concatenación con +
    ----------------------------------------

    Usando operador + (requiere un &str en el segundo operando).
    El primero se mueve → ya no se puede usar después.
    */

    let s8 = String::from("Hola ");
    let s9 = String::from("Mundo");
    let s10 = s8 + &s9; // s8 se mueve, s9 se presta

    println!("Concatenado con +: {}", s10);

    /*
    ----------------------------------------
        Formato
    ----------------------------------------
    */

    let nombre = "Jhon";
    let saludo = format!("Hola, {}!", nombre); // no mueve nada
    println!("{}", saludo);
}
