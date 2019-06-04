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
    

}
