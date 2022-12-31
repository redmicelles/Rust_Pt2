#![deny(clippy::all)]

fn main() {
    let mut values: Vec<i32> = vec![1, 2, 3, 4, 5];

    for value in &values{
        println!("value is {}", value);
    }

    let itr = values.iter();
    let sum: i32 = itr.sum();
    println!("Sum is {}", sum);

    values.push(6);
    //create a new iterator
    let itr2 = values.iter();
    let sum2: i32 = itr2.sum();
    println!("New Sum is {}", sum2);

    //using the map funtion
    let squares: Vec<f64> = values.iter().map(|v| f64::from(*v) * f64::from(*v)).collect();
    println!("Vector of Squares is {:#?}", squares);

    //iter with owned values and filter
    let names: Vec<&str> = vec!["John", "Jane", "Mary", "Bob", "Tom"];
    let mut name_itr = names.into_iter();
    
    println!("{:?}", name_itr);
    for name in name_itr.by_ref().filter(|name| name.len() == 3){
        println!("{}", name);
    } //this loop definite moves the values of the iterato
    println!("{:?}", name_itr);

    // let new_vec: Vec<&str> = name_itr.by_ref().take(3).collect();
    // println!("New vector {:#?}", new_vec);
    
}
