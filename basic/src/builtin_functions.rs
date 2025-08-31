fn vector_demo() {
    /*
    ----------------------------------------
        Metodos basicos
    ----------------------------------------
    */
    let mut v1 = Vec::new();
    for i in 0..9 {
        v1.push(i * 10);
    }

    println!("{:?}", v1);
    println!("Primer elemento: {} {}", v1[0], v1.first());
    println!("Tamaño del vector: {}", v1.len());

    v1.pop();

    println!("Último elemento: {:?}", v1.last());
    println!("Tamaño del vector: {}", v1.len());
    println!("¿Está vacío el vector? {}", v1.is_empty());
    println!("Elemento 2: {}", v1.get(2).unwrap());

    // inicializar
    let mut v2 = vec![0; 5]; // [0, 0, 0, 0, 0]
    let mut v2: Vec<i32> = (1..6).collect();
    println!("{:?}", v2); // [1, 2, 3, 4, 5]


    /*
    ----------------------------------------
        Slicing
    ----------------------------------------

    Hace referencia a una porción del vector sin copiar los datos. 
    Si se desea copiar, se puede hacer con el método to_vec().
    */

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

    /*
    ----------------------------------------
        Iteradores
    ----------------------------------------
    */

    /*
    Iterar SOLO LECTURA.
    tanto for x in &v4 como for x in v4.iter() son equivalentes.
    */
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
