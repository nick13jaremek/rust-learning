fn main() {
    assert_eq!('*'.is_alphabetic(), false);
    assert_eq!('β'.is_alphabetic(), true);
    assert_eq!('8'.to_digit(10), Some(8)); // Convert '8' to base 10 digit
    assert_eq!('ಠ'.len_utf8(), 3);
    assert_eq!(std::char::from_digit(2, 10), Some('2')); // Convert 2 in base 10 to character
    assert_eq!(std::char::from_digit(0b0001, 2), Some('1')); // Convert 1 in base 2 (binary) to character
}
