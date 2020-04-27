fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    // calling function
    another_function(5);

    // expression in the new scope to evaluate y value
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    // returning values from function
    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(11);
    println!("The value of x is: {}", x);
}
