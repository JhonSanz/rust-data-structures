/*
estructuras de datos concurrentes

Dashmap
crossbeam::queue::SegQueue, ArrayQueue
crossbeam, flume, evmap


*/



use std::thread;
use std::time::Duration;

fn ejemplo_1() {
    // Creamos un hilo separado
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Hilo secundario: {}", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    // Mientras tanto, el hilo principal sigue ejecutando
    for i in 1..3 {
        println!("Hilo principal: {}", i);
        thread::sleep(Duration::from_millis(1000));
    }

    // Esperamos a que el hilo secundario termine
    handle.join().unwrap();

    println!("Programa terminado ✅");
}
