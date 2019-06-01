// These declarations bring into scope the Write and FromStr traits.
// These allow to use the write and from_str methods
use std::io::Write;
use std::str::FromStr;

fn main() {
    let mut numbers: Vec<u64> = Vec::new();

    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg)
                     .expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
        std::process::exit(1);
    }

    let mut d = numbers[0];
    // Borrow items from item 1 to end: get references to items
    for m in &numbers[1..] {
        d = gcd(d, *m); // Deference iterated item with * (star operator)
    }

    println!("The greatest common divisor of {:?} is {}",
             numbers, d);
}

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
