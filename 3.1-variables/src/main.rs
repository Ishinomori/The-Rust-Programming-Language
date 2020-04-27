fn main() {
    // SOME BASICS
    // variable have to be mutable to reassign value
    let mut x = 5;
    println!("x: {}", x);
    x = 6;
    println!("x: {}", x);

    // constants
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS: {}", MAX_POINTS);

    // shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("x: {}", x);


    let x: i32 = 64_i32;
    println!("x i32: {}", x);

    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index];

    println!("The value of element is: {}", element); // panic!
}
