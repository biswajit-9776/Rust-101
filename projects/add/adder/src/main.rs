extern crate add_one;
extern crate add_two;

struct SmartPointer {
    data: String,
}
impl Drop for SmartPointer {
    fn drop(&mut self) {
        println!("Dropping SmartPointer with data: {}", self.data);
    }
}
enum List<'a> {
    Cons(i32, &'a Box<List<'a>>),
    Nil,
}
enum p {
    Cons(&String),
    
}
fn main() {
    println!("2 + 1 = {}", add_one::add_one(2));
    println!("2 + 1 = {}", add_two::add_two(2));

    let s = SmartPointer {
        data: String::from("my stuff"),
    };
    drop(s);
    println!("SmartPointer created, now going out of scope.");
    let nil = List::Nil;
    let a = List::Cons(5, Box::new(List::Cons(10, Box::new(nil))));
    let b = List::Cons(3, &Box::new(a));
    let c = List::Cons(4, &Box::new(a));
}
