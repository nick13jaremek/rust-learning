fn main() {
    println!("Some examples of basic type variables with values");
    println!("--------------------------------------------------");

    println!("INTEGER");
    let unsigned_8: u8 = 255;
    println!("Unsigned 8: {}", unsigned_8);

    let unsigned_16: u16 = 65530;
    println!("Unsigned 16: {}", unsigned_16);
    
    let unsigned_32: u32 = 4_294_967_290;
    println!("Unsigned 32: {}", unsigned_32);

    let unsigned_64: u64 = 18_432_291_209_300;
    println!("Unsigned 64: {}", unsigned_64);

    let usized: usize = 10_000_000; // Machine dependent (32bits or 64bits)
    println!("Usize: {}", usized); 
    
    let signed_8: i8 = -100;
    println!("Signed 8: {}", signed_8);

    let signed_16: i16 = -25_000;
    println!("Signed 16: {}", signed_16);
    
    let signed_32: i32 = -1_500_987;
    println!("Signed 32: {}", signed_32);

    let signed_64: i64 = -8_459_100_551;
    println!("Signed 64: {}", signed_64);

    let isized: isize = -10_100_200; // Machine dependent (32bits or 64 bits)
    println!("Signed size: {}", isized);

    let hexadecimal = 0xcafeu32; // Values prefixed with 0x are hex values
    println!("Hexadecimal: {}", hexadecimal);

    let binary = 0b0010_1010; // Bits can be separated by underscores just like in the case of integers. Values prefixed with 0b are binary values.
    println!("Binary: {}", binary);

    let octal = 0o106; // Values prefixed with 0o are octal numbers
    println!("Octal: {}", octal);

    let byte_literal = b'A';
    println!("Byte literal: {}", byte_literal); // The same as 65u8

    let special_byte_literal = b'\t'; // Special characters such as tabs or backslashes must be prepended with a backslash character
    println!("Escaped byte literal: {}", special_byte_literal);

    // COVERSIONS
    assert_eq!(10_i8 as u16, 10_u16);
    assert_eq!(-1_i16 as i32, -1_i32); // Sign-extended

    assert_eq!(1000_i16 as u8, 232_u8); // Trucated: equal to 1000 modulo 2^N where N = number of bits of destination type (in this case it is 8).

    // METHODS USAGE
    // Using methods on numeric values requires suffixing such values with the numeric type,
    // otherwise Rust cannot look up a value's method (the value's type is needed in advance)
    assert_eq!(2u16.pow(4), 16);
    assert_eq!((-4i32).abs(), 4);

    println!("FLOATING POINT");
    println!("--------------------------------------------------");

    let simple_float = -1.5625;
    println!("Simple float: {}", simple_float);

    let f32_value = 40f32;
    println!("Float32: {}", f32_value);

    let f64_value = 9.109_383_56e-31f64;
    println!("Float64: {}", f64_value);

    assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.);

    // BOOLEAN
    assert_eq!(false as i32, 0);
    assert_eq!(true as i32, 1);
    // Cannot convert the other way around: 0 as bool.
    
    println!();
    println!("CHARACTERS");
    println!("--------------------------------------------------");

    let single_char = 'X'; // Chars are enclosed in SINGLE QUOTES, NOT DOUBLE QUOTES
    println!("Single char: {}", single_char);

    let unicode_char = '∭';
    println!("Unicode char: {}", unicode_char);
    
    // TUPLES
    println!();
    println!("TUPLES");
    println!("--------------------------------------------------");

    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");

    let tuple_value = ("Brazil", 1985); // Type (&str, <numeric>)
    println!("Tuple value: {:?}", tuple_value);

    let single_item_tuple = ("Lonely hears",);
    println!("Single item tuple: {:?}", single_item_tuple);
    
    println!();
    println!("ARRAYS");
    println!("--------------------------------------------------");

    // Arrays are used to store a fixed-length list of items
    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    assert_eq!(lazy_caterer[3], 7); // Index-based access
    println!("Array of 6 items: {:?}", lazy_caterer);

    let bool_list = [true; 10]; // Array of 10 items, all true boolean values
    println!("Bool list: {:?}", bool_list);

    let mut chaos = [3, 5, 4, 1, 2];
    chaos.sort(); // Can be sorted because variable is mutable
    println!("Sorted chaos: {:?}", chaos);

    println!();
    println!("VECTORS");
    println!("--------------------------------------------------");

    // Vectors are used as dynamic arrays. Allocation is done on the HEAP.
    let mut v = vec![2, 3, 5, 7]; // Useful macro to create vector
    v.push(11); // Can push items to vector because it is a vector AND it is a mutable variable
    println!("Vector: {:?}", v);

    let mut vector_str = Vec::new();
    vector_str.push("step");
    vector_str.push("on");
    vector_str.push("floor");
    assert_eq!(vector_str, vec!["step", "on", "floor"]);

    // Can create vectors with capacity specified up front
    let mut vector_with_capacity = Vec::with_capacity(2);
    assert_eq!(vector_with_capacity.len(), 0);
    assert_eq!(vector_with_capacity.capacity(), 2);

    vector_with_capacity.push(1);
    vector_with_capacity.push(2);
    assert_eq!(vector_with_capacity.len(), 2);
    assert_eq!(vector_with_capacity.capacity(), 2);

    // When capacity is surpassed, vector is reallocated with twice the capacity
    vector_with_capacity.push(3);
    vector_with_capacity.push(4);
    assert_eq!(vector_with_capacity.len(), 4);
    assert_eq!(vector_with_capacity.capacity(), 4);

    // Popping items from vector
    let mut people = vec!["carmen", "miranda"];
    assert_eq!(people.pop(), Some("miranda")); // Note the return type of pop() is Option<T> (Some(value), None)
    assert_eq!(people.pop(), Some("carmen"));
    assert_eq!(people.pop(), None);

    // SLICES
    println!();
    println!("SLICES");
    println!("--------------------------------------------------");
    let v_vec: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a_arr: [f64; 4] = [0.0, 0.707, 1.0, 0.707];

    // These are slice references to vector/array references
    let sv: &[f64] = &v_vec; // This is a slice to a vector
    let sa: &[f64] = &a_arr; // This is a slice to an array
    println!("Vector slice: {:?}", sv);
    println!("Array slice: {:?}", sa);

    // STRINGS
    println!();
    println!("STRINGS");
    println!("--------------------------------------------------");

    // This is a string literal. It must be enclosed in double quotes.
    let speech = "\"Ouch!\" said the well.\n";
    println!("String literal: {}", speech);

    // Raw strings are defined by preprending their values with 'r'
    let default_win_install_path = r"C:\Program Files\Gorillas";
    println!("Raw string: {}", default_win_install_path);
    println!(r###"
        This raw string started with 'r###'.
        Therefore it does not end until we reach a quote mark ('"') followed immediately by three pound signs ('###'):"###);

    // Byte strings: slices of u8 values, therefore bytes, not Unicode text.
    let method = b"GET";
    assert_eq!(method, &[b'G', b'E', b'T']);

    // A String has a resizable buffer holding UTF-8 text.
    // It can be thought of as a Vec<u8> that is guaranteed to hold well-formed UTF-8.
    let noodles = "noodles".to_string(); // String literal converted to String
    let oodles = &noodles[1..]; // &str: pointer to a String value
    let poodles = "þoø→"; // String literal or &str
    println!("Oodles: {:?}", oodles);
    println!("Poodles: {:?}", poodles);
    // &str is more appropriate for function arguments when the caller should be allowed to pass
    // either an &str or a String.
}
