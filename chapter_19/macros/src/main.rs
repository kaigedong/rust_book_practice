use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(Debug, HelloMacro)]
struct Test;

fn main() {
    let _a: Vec<u32> = vec![1, 2, 3];
    println!("Hello, world!");

    let b = Test;
    Test::hello_macro();
    // println!("Hi, this is my first proc macro {:?}");
}
