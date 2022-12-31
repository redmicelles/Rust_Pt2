#![deny(clippy::all)]

fn get_user_name() -> Result<String, ()> {
    Ok("John".to_string())
    //Err(())
}

fn main() {
    let value: Result<&str, Box<dyn std::error::Error>> = Ok("Hello");

    match value {
        Ok(value) => println!("{}", value),
        Err(error) => println!("{}", error),
    }

    //none unit type error
    let value: Result<&str, ()> = Err(());

    match value {
        Ok(value) => println!("{}", value),
        Err(_) => println!("Some error occurred"),
    }

    //use expect
    let user_name = get_user_name().expect("failed to get username");
    println!("{}", user_name);

    // check if a function returns Ok or Err Result enum

    let resp_type1 = get_user_name().is_ok();
    let resp_type2 = get_user_name().is_err();
    println!("{resp_type1}");
    println!("{resp_type2}");
}
