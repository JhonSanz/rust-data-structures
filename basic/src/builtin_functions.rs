fn builtin_demo() {
    let s = String::from("Hola Rust");
    println!("len = {}", s.len());
    println!("is_empty = {}", s.is_empty());
    println!("contains 'Rust'? {}", s.contains("Rust"));
    println!("replace: {}", s.replace("Rust", "Mundo"));

    let nums = vec![1, 2, 3, 4, 5];
    println!("len = {}", nums.len());
    println!("first = {:?}", nums.first());
    println!("last = {:?}", nums.last());
    println!("iter sum = {}", nums.iter().sum::<i32>());
}