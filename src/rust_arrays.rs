#[allow(dead_code)]
pub fn run() {
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{arr:?}");

    arr[2] = 90;

    println!("{arr:?}");
}
