// Works on slices of type f64
fn print(n: &[f64]) {
    for elt in n {
        println!("{}", elt);
    }
}

fn main() {
    let vector: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let array: [f64; 4] = [0.0, -0.707, -1.0, -0.707];

    let vector_slice: &[f64] = &vector;
    let array_slice: &[f64] = &array;

    println!("Printing vector slice...");
    print(&vector_slice); // Works on vectors

    println!("Printing array slice...");
    print(&array_slice); // Works on arrays

    println!("Printing first 2 elements of vector..");
    print(&vector_slice[0..2]);

    println!("Printing items in array slice from second item onwards...");
    print(&array_slice[2..]);

    println!("Printing items 1 and 2 of vector slice...");
    print(&vector_slice[1..3]);
}
