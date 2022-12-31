#![deny(clippy::all)]

fn say_hello_world() -> String {
    String::from("Hello, world!")
}

fn say_hello(to_person: &str) -> String{
    format!("Hello, {}!", to_person)
}

fn process_name(name: &str, callback: fn(&str) -> ()){
    callback(name);
}

fn main() {
    let message = say_hello_world();
    println!("{message}");
    let message2 = say_hello("Dexter");
    println!("{}", message2);

    //inline function say hello
    let say_hello_inline = |name: &str| format!("Hello {name}!");
    let message3 = say_hello_inline("Nathan");
    println!("{}", message3);

    //inline function get input
    let ask_for_age = || {
        let mut age: String = String::new();
        println!("How old are you: \n");
        std::io::stdin().read_line(&mut age).expect("Failed");
        let your_age: u32 = age.trim().parse::<u32>().expect("Please enter a number");
        format!("You will be {} in ten years", your_age + 10)
    };

    let age_in_ten_years = ask_for_age();
    println!("{age_in_ten_years}");

    //function whose arguement is another function

}