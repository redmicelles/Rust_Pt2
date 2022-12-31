#![deny(clippy::all)]

fn main() {
    let _value = Some(10);
    let name: Option<&str> = Some("Seyi");

    match name {
        Some(name) => println!("Hello, {}", name),
        None => println!("There is no name")
    }

    //mutable options
    let mut age: Option<i32> = Some(20);

    //use unwrap to print content of Option
    println!("{:?}", age.unwrap());

    match age.as_mut() {
        Some(age) => {
            *age *= 2;
            println!("{:?}", age);
        },
        None => println!("No age")
    }

    //using unwrap_else
    // let fname: Option<&str> = Some("Daniel");
    // let unwrap = fname.unwrap_or_else(||"Dexter Daniel");
    // println!("Surname is {}", unwrap);

    // let k = 10;
    // assert_eq!(Some(4).unwrap_or_else(|| 2 * k), 4);
    // assert_eq!(None.unwrap_or_else(|| 2 * k), 20);

    //check if an option has a value or not

    let mname:Option<&str> = None;
    if mname.is_none(){
        println!("Name is empty");
    }

    if age.is_some(){
        println!("Age is not empty")
    }
    // unwrap or default
    let mname_unwrap = mname.unwrap_or_default();
    println!("{}", mname_unwrap);

    // using map with option
    let double_age = age.map(|age| age * 8);
    println!("Double Age is {}", double_age.unwrap_or_default());
}
