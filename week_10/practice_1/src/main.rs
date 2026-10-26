fn main() {
    
    let v = vec![101, 250, 330, 400];
    // vector v owns the objecgt in heap

    // only a single variable owns the heap menory at any given time
    let v2 = v.clone();
    // here two variables owns heap value,
    // two pointers to the same content is not allowed in rust

    // Rust is very smart in terms of memory access, so it detects a race condition  
    // as two variable point to same heap

    println!("{:?}", v);
}
