fn main() {
    // Build a vector using the vec! macro
    let mut nums = vec![2, 3, 5, 7];
    assert_eq!(nums.iter().fold(1, |a, b| a * b), 210);

    nums.insert(3, 35); // Insert 35 at index 3, note it shifts items to the right
    assert_eq!(nums, vec![2, 3, 5, 35, 7]);
    nums.remove(1); // Remove value at index 1
    assert_eq!(nums, vec![2, 5, 35, 7]);

    // Build a vector one-item-at-a-time using the new constructor
    let mut v = Vec::new();
    v.push("step");
    v.push("on");
    v.push("no");
    v.push("pets");
    assert_eq!(v, vec!["step", "on", "no", "pets"]);

    // Slice-defined methods can also be applied on vectors
    let mut palindrome = vec!["a man", "a plan", "a canal", "panama"];
    palindrome.reverse(); // Slice-defined method: reverse
    // Reasonable, yet disappointing result
    assert_eq!(palindrome, vec!["panama", "a canal", "a plan", "a man"]);

    // Building vector with specific capacity
    let mut cap = Vec::with_capacity(2);
    assert_eq!(cap.len(), 0); // Reasonable: no items yet
    assert_eq!(cap.capacity(), 2); // Vector starts with room for 2 items

    cap.push(1);
    cap.push(2);
    assert_eq!(cap.len(), 2);
    assert_eq!(cap.capacity(), 2);

    cap.push(3);
    assert_eq!(cap.len(), 3);
    assert_eq!(cap.capacity(), 4); // Vector doubled its capacity!

    // Popping values
    let mut names = vec!["carmen", "miranda"];
    assert_eq!(names.pop(), Some("miranda"));
    assert_eq!(names.pop(), Some("carmen"));
    assert_eq!(names.pop(), None); // Empty vector
}
