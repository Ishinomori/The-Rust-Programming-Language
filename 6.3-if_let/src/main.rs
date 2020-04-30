#![allow(unused_variables)]
#![allow(dead_code)]

fn main() {
    // what we want to do
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // simplified with if let
    if let Some(3) = some_u8_value {
        // pattern matching instead of bool condition
        println!("three");
    }
}
