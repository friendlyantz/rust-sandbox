// Fix the error below with least amount of modification to the code
fn main() {
    mutation();
    shadowing();
    scopes();
    unused_var();
}

#[allow(unused_variables)]
fn unused_var() {
    let x = 1;
}

fn scopes() {
    let x = 1;
    {
        let y: i32 = 5;
        println!("Inner scope value of x is {} and value of y is {}", x, y);
    }
    let y: i32 = 6;

    println!("Inner scope value of x is {} and value of y is {}", x, y);
}

fn mutation() {
    let mut x: i32 = 1;
    x += 4;

    assert_eq!(x, 5);
    println!("Mutation Success!");
}

fn shadowing() {
    let mut x: i32 = 1;
    println!("{x}");
    x = 7;
    println!("{x}");
    // Shadowing and re-binding
    let mut x = 2;
    x += 3;

    let y = 4;
    println!("{y}");
    // Shadowing
    let y = "I can also be bound to text!";

    println!("Shadowing Success! {} {}", x, y);
}
