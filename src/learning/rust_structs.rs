#[allow(dead_code)]
pub fn run() {
    struct User {
        name: String,
        age: u32,
        salary: u64
    }

    let mut user_1: User = User {
        name: String::from("Krish"),
        age: 19,
        salary: 0
    };

    user_1.age = 20_000;

    println!("Name: {} | Age: {} | Salary: {}", user_1.name, user_1.age, user_1.salary);
}