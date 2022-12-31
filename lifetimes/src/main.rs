#![deny(clippy::all)]

fn get_full_name() -> &'static str {
    "John Doe"
}

fn get_random_name<'l>(a: &'l str, b: &'l str) -> &'l str{
    format!("{a} {b}");
    a
}

fn main() {
    let full_name = get_full_name();
    println!("Hello, {}", full_name);

    let rand_name = get_random_name("Elyon", "Daniel");
    println!("{}", rand_name)
}
