# learn RUST

## after PJ Klimek meet on 2023-10-07

cargo run // execute app
cargo fmt // format code
cargo test // exec test macro inside code - for tests

rockert.rs // for API in Rust
systemd // for system deamon

systemctl // unix cmd for deamons
systemctl show 

// ---

## let, const

let x: i32 = 1 // Immutable var
let mut y: i32 = 1 // Mutable var

let text: &str = "this is string" // str vs String ?
const SOLID_VAR = 1; // this is const

## conditionals
// if statement

let age: i32 = 19;
    if (age >= 1) && (age <= 17) {
        println!("you are not adult having {}", age);
    } else if (age == 18) {
        println!("just became adult!");
    } else {
        println!("you have {} years", age)
    }

// if inside let
let can_vote: bool = if age >= 18 { true } else { false };
println!("Can vote: {}, in age {}", can_vote, age);

//