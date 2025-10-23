#[allow(dead_code)]
pub fn run() {
    let mut tuple: (i32, &str, i32) = (21, "Hello World", -3);

    println!("{tuple:?}");

    tuple.1 = "Test";

    println!("{tuple:?}");
}
