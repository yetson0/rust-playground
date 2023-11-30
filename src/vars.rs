pub fn run() {
    const THIS_IS_CONST: u16 = 1_000; // define constant
    let age_string: &str = "50"; // string variable
    let simple_variable = 'V'; // character variable
    println!("constant = {THIS_IS_CONST}, age_string = {age_string}, char = {simple_variable}");

    let name = "yetson"; // immutable variable by default
    let mut age: i32 = 19; // mutable variable
    age = age + 1;

    let (my_name, my_age) = ("Yetson", 44); // multiple variables

    // if statement
    if (age >= 1) && (age <= 17) {
        println!("not adult having {}", age);
    } else if age == 18 {
        println!("just became adult!");
    } else {
        println!("{name} you have {} years", age)
    }

    // match statement
    match age {
        1..=17 => println!("not adult having {}", age),
        18 => println!("adult"),
        18..=i32::MAX => println!("you have {} years", age),
        _ => println!("everything else"),
    }

    let can_vote: bool = if age >= 18 { true } else { false };
    println!("Can vote: {}, in age {}", can_vote, age);
}
