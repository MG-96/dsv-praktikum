enum para_options {
    int64(i64),
    int32(i32),
    string(String),
}
enum simple {
    monday,
    tuesday,
}
fn genericFunction(some: para_options) -> para_options {
    return some
}


fn main() {
    println!("Hello, world!");

    let number = para_options::int64(123);

    let returned = genericFunction(number);

}
