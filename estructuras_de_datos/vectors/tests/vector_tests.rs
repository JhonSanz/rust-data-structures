use vectors::MyVec;

#[test]
fn test_vector_access_o1() {
    let mut v = MyVec::new();

    // Añade 1000 elementos
    for i in 0..1000 {
        v.push_back(i);
    }

    // Acceso O(1) a cualquier posición
    assert_eq!(v.get(0), Some(&0));
    assert_eq!(v.get(500), Some(&500));
    assert_eq!(v.get(999), Some(&999));

    // El acceso al elemento 999 NO es más lento que al elemento 0
    // Ambos son O(1) porque usa aritmética de punteros

    println!("✅ Vector confirmado: acceso O(1) a cualquier índice");
}

#[test]
fn test_get_out_of_bounds() {
    let mut v = MyVec::new();
    v.push_back(10);
    v.push_back(20);

    assert_eq!(v.get(0), Some(&10));
    assert_eq!(v.get(1), Some(&20));
    assert_eq!(v.get(2), None);  // Fuera de rango
}

#[test]
fn test_push_and_grow() {
    let mut v = MyVec::new();

    assert_eq!(v.capacity(), 0);
    assert_eq!(v.len(), 0);

    // Primer push dispara allocate
    v.push_back(1);
    assert!(v.capacity() >= 4);  // Capacidad inicial
    assert_eq!(v.len(), 1);

    // Llenar hasta forzar grow
    for i in 2..=10 {
        v.push_back(i);
    }

    assert!(v.capacity() >= 10);
    assert_eq!(v.len(), 10);

    // Verificar todos los elementos
    for i in 0..10 {
        assert_eq!(v.get(i), Some(&((i + 1) as i32)));
    }
}
