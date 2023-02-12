use std::fs;
fn main() {
    let list = fs::read_to_string("list.txt").expect("file not found...");
    println!("{}", list);
}
