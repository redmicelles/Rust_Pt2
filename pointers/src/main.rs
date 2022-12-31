#![deny(clippy::all)]

fn print_integer(value: &i32){
    println!("{}", value);
}

fn main() {
    // let age: Box<i32> = Box::new(22);
    // let twice = *age * 2;
    // println!("{}", twice);
    // let actual_age = &age;
    // let ref_to_value = actual_age.deref();
    // let other = *(age.deref());

    // println!("actual_age = {}", actual_age);
    let value: Box<i32> = Box::new(10);
    // value is implicitly derefrenced by the print_integer function
    print_integer(&value);
}
