fn main() {
    // Rust drops (frees) values whenever they go out of scope
    print_padovan();

    // A Box<T> is a pointer to a value of type T stored on the heap.
    // Calling Box::new(v) allocates some heap space, moves the value v into it and returns a Box
    // pointing to the heap space
    let point = Box::new((0.625, 0.5)); // Point allocated into heap
    let label = format!("{:?}", point); // label allocated here
    assert_eq!(label, "(0.625, 0.5)");

    struct Person { name: String, birth: i32 };
    let mut composers = Vec::new();
    composers.push(Person { name: "Palestrina".to_string(), birth: 1525 });
    composers.push(Person { name: "Dowland".to_string(), birth: 1563 });
    composers.push(Person { name: "Lully".to_string(), birth: 1632 });
    for composer in &composers { // Borrow reference to composers
        println!("{}, born {}", composer.name, composer.birth);
    }

    // Moves and control flow
    let x = vec![10, 20, 30];
    let c = true;
    if c {
        f(x); // Ok to move from x here
    } else {
        g(x); // Also ok to move from x here
    }
    // Calling here some other method passing x as its argument would be invalid because ownership
    // over x was passed to either method f or method g
    

    // Build a vector of strings "101", "102", ..., "105"
    let mut v = Vec::new();
    for i in 101 .. 106 {
        v.push(i.to_string());
    }

    // 1. Pop a value off the end of the vector
    let fifth = v.pop().unwrap();
    assert_eq!(fifth, "105");

    // 2. Move a value out of the middle of the vector and move the last element into its spot
    let second = v.swap_remove(1);
    assert_eq!(second, "102");

    // 3. Swap in another value for the one we're taking
    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");

    // Let's see what is left of our vector
    assert_eq!(v, vec!["101", "104", "substitute"]);
}

fn f(x: Vec<i32>) {
    println!("f-X: {:?}", x);
}

fn g(x: Vec<i32>) {
    println!("g-X: {:?}", x);
}
fn print_padovan() {
    let mut padovan = vec![1, 1, 1]; // allocated here
    for i in 3..10 {
        let next =  padovan[i - 3] + padovan[i - 2];
        padovan.push(next);
    }
    println!("P(1..10) = {:?}", padovan);
} // padovan dropped here
