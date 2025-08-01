extern crate add_one;
extern crate add_two;
fn main() {
    println!("2 + 1 = {}", add_one::add_one(2));
    println!("2 + 1 = {}", add_two::add_two(2));
}
