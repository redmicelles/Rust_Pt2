#![deny(clippy::all)]


struct Person<'a> {
    first_name: &'a str,
    last_name: &'a str,
}

impl<'a> Person<'a> {
    //follows Lifetime rule 3
    fn first_char_of_first_name(&self) -> &str {
        &self.first_name[0..1]
    }

    fn get_full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}
fn main() {
    // let p1: Person = {
    //     first_name: "Seyi";
    // }
}
