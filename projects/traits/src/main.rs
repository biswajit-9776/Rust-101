use std::fmt::format;

trait Summary {
    fn Summarize(&self) -> String {
        format!("@{}", self.summarize_author())
    }
    fn summarize_author(&self) -> String;
}
struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("Read more from: {}", self.username)
    }
}

fn largest<T: PartialOrd + Copy> (list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn main() {
    let tweet = Tweet {
        username: String::from("horsebooks"),
        content: String::from("Blackbread"),
        reply: true,
        retweet: false,
    };
    println!("{}", tweet.summarize_author());
    let list1 = vec![1,2,3,4];
    let mut list2 = "hello";
    let h = &list1[0..1];
    println!("{:?}, {:?}", h, list2);
    println!("{:?}", largest(&list1));
    println!("{:?}", largest(list2.as_bytes()) as char);
}
