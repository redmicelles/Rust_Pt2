#![deny(clippy::all)]
#![allow(dead_code)]

use std::fmt;

// #[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8
}

//implement display for PErson
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} is {} years old", self.first_name, self.last_name, self.age)
    }
}

trait HasName {
    fn first_name(&self) -> &str;
    fn last_name(&self) -> &str;
}

//trait binding, if an object implements HasName,
//it automatically implements the Has complete name trait
trait HasCompleteName
where
    Self: HasName,
{
    // fn complete_name(&self) -> String {
    //     format!("{}, {}", self.last_name(), self.first_name())
    // }
    fn complete_name(&self) -> String;
}

impl<T> HasCompleteName for T
where
    T: HasName,
{
    fn complete_name(&self) -> String {
        format!("{}, {}", self.last_name(), self.first_name())
    }
}

trait  HasFullName {
    fn full_name(&self) -> String;
}

impl HasFullName for Person {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

impl HasName for Person {
    fn first_name(&self) -> &str {
        &self.first_name
    }

    fn last_name(&self) -> &str {
        &self.last_name   
    }
}


trait CanIntializeWithFullname {
    fn new(full_name: &str) -> Self;
}

impl CanIntializeWithFullname for Person {
    fn new(full_name: &str) -> Self {
        let parts: Vec<&str> = full_name.split(' ').collect();
        Person {
                first_name: parts[0].to_string(),
                last_name: parts[1].to_string(),
                age: 18
            }
    }
}

trait CanRun {
    fn run(&self);
}

impl CanRun for Person {
    fn run(&self) {
        //todo..
    }
}


// Traits can be passed as function parameters, so as to impose the 
// trait implementation on any entity that wants to use the function
fn print_full_name_and_age(value: &impl HasFullName) {
    println!("{}", value.full_name());
}

//using generics to implement the feature above with multiple traits
fn print_details<T>(value: &T) 
where
    T: HasFullName + CanRun,
{
    println!("{}", value.full_name());
    value.run()
}

fn main() {
    // let person: Person = Person {
    //     first_name: "John".to_string(),
    //     last_namae: "Doe".to_string(),
    //     age: 30,
    // };
    // println!("{:#?}", person);

    let person2 = Person::new("Dexter Daniel");
    //print person2 using the formatted display by display fromatter
    println!("{}", person2);
    print_full_name_and_age(&person2);

    print_details(&person2);
    println!("{}", person2.complete_name());
}
