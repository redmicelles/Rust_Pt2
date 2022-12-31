#![deny(clippy::all)]
#![allow(dead_code)]

use std::cell::Cell;

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: Cell<u8>, //a cell is a kind of pointer that allow internal mutability
}

impl Person<'_> {
    fn increment_age(&self) {
        self.age.set(self.age.get() + 1);
    }
}

fn main() {
    let person: Person = Person {
        name: "John",
        age: Cell::new(20),
    };

    person.increment_age();
    println!("{:#?}", person)
}
