#![deny(clippy::all)]

fn main() {
   let values = ("Seyi", "Daniel", 30);
   println!("{:?}", values);
   //tuple unpacking
   let (val1, val2, val3) = values;
   println!("{}, {}, {}", val1, val2, val3);
   
   //Tuple unpack only certain values
   let (_, sname, _) = values;
   println!("{}", sname);

   //vectors
   let values1: [i32; 2] = [10, 20];
   let _squares = values1.iter().map(|x| x * x);
   
   //mutable vectors from vector marcros
   let mut values2: Vec<i32> = vec![1, 2, 3, 4, 5];

   //push value vector
   values2.push(6);
   println!("{:?}", values2);

   //extend vector 
   values2.extend_from_slice(&[11, 12, 13, 14]);
   println!("{:?}", values2);

   //append vector, this moves the value of one vector into another
   let mut values3: Vec<i32> = vec![20, 21, 22, 23, 35];
   values2.append(&mut values3);
   println!("{:?}", values3);
   println!("{:?}", values2);

   //check if vector contains a value
   if values2.contains(&35) {
      values2.pop();
      println!("{:?}", values2)
   }else{
      values2.push(35);
      println!("{:?}", values2)
   }
   
   if values3.is_empty(){
      println!("Vector is empty");
   }else{
      println!("Vector has some items");
   }
}
