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
