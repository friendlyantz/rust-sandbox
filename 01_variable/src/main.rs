// Fix the error below with least amount of modification to the code
fn main() {
    let mut x: i32 = 1;
    {
        let y: i32 = 5;
        println!("Inner scope value of x is {} and value of y is {}", x, y);
    }
    let y: i32 = 6;

    println!("Inner scope value of x is {} and value of y is {}", x, y);
    x += 4;

    assert_eq!(x, 5);
    println!("Success!");
}
