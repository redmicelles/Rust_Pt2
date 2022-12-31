#![deny(clippy::all)]
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Person {
    firstname: String,
    lastname: String,
    age: u8,
}

fn main() {
    let mut values: HashMap<&str, &str> = HashMap::new();
    values.insert("key", "value");
    println!("{:?}", values);
    
    //read values unsafely
    println!("{:?}", values["key"]);

    //read values unsafely
    println!("{:?}", values.get("key"));

    //Check if harshmap contains key
    if values.contains_key("key"){
        println!("Key exists");
    }

    values.insert("key2", "value2");
    values.insert("key3", "value3");

    //update a key's value using insert
    values.insert("key3", "key33");

    //insert using the entry api
    values.entry("key4").or_insert("value4");

    for (&k, &v) in &values {
        println!("{}, {}", k, v)
    }

    let person1 = Person{
        firstname: "Elyon".to_string(),
        lastname: "Daniel".to_string(),
        age: 2
    };

    // let mut values2: HashMap<&str, &Person> = HashMap::new();
    // values.insert(&person1.firstname, &person1);
}
