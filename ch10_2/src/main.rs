#![deny(clippy::all)]

fn get_first_name() -> Result<String, ()> {
    Ok("Enedabojo".to_string())
}

fn get_last_name() -> Result<String, ()> {
    Ok("Ellah-Daniel".to_string())
    // Err(())
}

fn get_full_name() -> Result<String, ()> {
    let first_name = get_first_name()?;
    let last_name = get_last_name()?;
    Ok(format!("Your name is {} {}", first_name, last_name))
}

fn main() {
    let full_name = get_full_name();
    // println!("{}", full_name.unwrap());
    match full_name {
        Ok(name) => println!("{name}"),
        Err(_) => println!("Ecountered some error!!"),
    }
}
