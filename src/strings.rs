// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    // multiple ways to create strings
    let s = "Hello".to_string();
    let s = String::from("world");
    let s: String = "also this".into();
    // let mut hello= "Hello"; // primitive str type

    let mut hello = String::from("Hello ");

    hello.push('W'); // push for char
    hello.push_str("orld!"); // push_str for string

    println!("{}", hello);
    println!("Len {}", hello.len());
    println!("Capacity {}", hello.capacity());
    println!("Is empty {}", hello.is_empty());
    println!("Contains {}", hello.contains("World"));
    println!("Replace {}", hello.replace("World", "There"));

    // loop throught string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len()); // check if len of s is 2
    assert_eq!(10, s.capacity()); // check capacity of string

    println!("{}", s);
}
