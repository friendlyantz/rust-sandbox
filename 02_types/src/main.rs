fn main() {
    mismatch();
    inline_type();
    type_func();
    hex_numbers();
    float_precision();
    asci_range();
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

fn hex_numbers() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111; // 1024 + 255 + 63 + 255
    assert!(v == 1597);

    println!("Success!");
}

fn float_precision() {
    let a = 0.1;
    let b = 0.2;
    let r;
    r = a + b;
    println!("{}", r);
    assert!(0.1_f32 + 0.2_f32 == 0.3_f32);
    assert!(0.1 as f32 + 0.2 as f32 == 0.3 as f32);

    println!("Success!");
}

// https://ascii-tables.com/
fn asci_range() {
    for c in 'a'..='z' {
        println!("{}", c as u8);
    }
}
