fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0); // This triggers a panic if assertion fails 
    while m != 0 {
        if m < n {
            let t: u64 = m; // Rust would infer the type if it were not provided
            m = n;
            n = t;
        }
        m = m % n;
    }
    n // This is a return value. Note it has no trailing semicolon
}

// This is an attribute that marks the following code block as a test
#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
                   3 * 7 * 11 * 13 * 19), 3 * 11);
}
