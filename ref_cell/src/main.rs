#![deny(clippy::all)]

use std::cell::RefCell;

fn main() {
    let ref_cell = RefCell::new(vec![1, 2, 3]);
    let mut mut_ref = ref_cell.borrow_mut();
    // let len = ref_cell.borrow().len();
    mut_ref.push(100);
    println!("{:#?}", mut_ref);
}
