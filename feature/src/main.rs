#[cfg(feature="some_condition")]
fn conditional_function() {
    println!("condition met!");
}

fn main() {
    conditional_function();
    println!("Hello, world!");
}