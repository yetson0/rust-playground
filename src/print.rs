pub fn run() {
    // print to console
    println!("Hello from print.rs file, yetson!");

    let x = 13;
    // let add_result = add(x, 3);

    // basic formatting
    println!("x = {}", x);

    // positional arguments
    println!("add = {0}", add(x, 2));

    // named arguments
    println!("add = {additive}", additive = add(x, 2));

    // placeholder traits
    println!("Bianry: {:b} Hex: {:x} Octal {:o}", 15, 15, 15);

    // placeholder for debug trait
    println!("{:?}", (12, true, "Its true"));

    // function definition
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }
}
