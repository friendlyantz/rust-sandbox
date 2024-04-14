fn main() {
    mismatch();
    inline_type();
    type_func();
}

#[allow(unused_variables)]
fn mismatch() {
    let x: u32 = 5;
    let mut y: u32 = 5;
    y += 1;
    println!("Success! {y}");

    y = x;

    let z = 10; // Type of z ?

    println!("Success! {y}");
}

fn inline_type() {
    let v: u16 = 38_u8 as u16; // 38 unsigned 8bit int

    println!("Success! {v}");
}

fn type_func() {
    let x: u32 = 5;
    assert_eq!("u32".to_string(), type_of(&x));

    println!("Success! type func");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
