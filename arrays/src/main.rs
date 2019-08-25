fn main() {
    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];
    let sieve = [true; 10000]; // Array with 10000 boolean true values

    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);
    assert_eq!(sieve.len(), 10000);

    // Note: most of the array methods are actually implemented for slices.
    // When calling a slice-defined method on an array, Rust implicitly
    // produces a reference to a slice of the array and executes the 
    // method passing this slice reference as an argument.
    let mut chaos = [3, 5, 4, 1, 2];
    chaos.sort(); // Note 'chaos' must be a mutable variable
    assert_eq!(chaos, [1, 2, 3, 4, 5]);
}
