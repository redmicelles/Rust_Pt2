#![deny(clippy::all)]
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    mothers_maiden_name: String,
}

//create tuples using structs
#[derive(Debug)]
struct Point(f64, f64, f64);

enum Direction{
    X,
    Y,
    Z,
}

impl Point {
    fn describe(&self) {
        println!("Point is at ({}, {}, {})", self.0, self.1, self.2);
    }

    fn move_point(&mut self, direction: Direction, distance: f64) {
        match direction{
            Direction::X => {
                self.0 += distance;
                println!("New Point is {:#?}", self);
            },
            Direction::Y => {
                self.1 += distance;
                println!("New Point is {:#?}", self);
            },
            Direction::Z => {
                self.2 += distance;
                println!("New Point is {:#?}", self);
            },
        }
    }
    
    //factory method
    fn create_origin_point() -> Point {
        Point(0.0, 0.0, 0.0)
    }

}

fn main() {
    let person1: Person = Person {
        name: "Elyon".to_string(),
        age: 2,
        mothers_maiden_name: "Ellah".to_string(),
    };
    println!("{} is {} years old. Child of Miss {}", person1.name, person1.age, person1.mothers_maiden_name);

    //struture update syntax
    let person2: Person = Person {
        name: "Nathan".to_string(),
        ..person1
    };
    println!("{:#?}", person2);

    //intstantiate tupl
    let mut point: Point = Point(1.2, 1.0, 2.0);
    point.describe();

    point.move_point(Direction::Y, 1.4);

    // create factory point
    let origin = Point::create_origin_point();
    println!("{:#?}", origin)
}
