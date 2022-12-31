#![deny(clippy::all)]
use std::rc::Rc;

fn main() {
    let array: Vec<String> = vec!["John".to_string(), "Jane".to_string()];
    let rc = Rc::new(array);
    //rc cloning
    let rc2 = Rc::clone(&rc);
    drop(rc);
    println!("{:?}", rc2);
    
}
