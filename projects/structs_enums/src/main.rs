use std::net::{IpAddr, Ipv4Addr};
fn main() {
    let mut user = User {
        name: String::from("Patrick"),
        email: String::from("patrick@gmail.com"),
        sign_in_count: 1,
        active: true,
    };
    user.name = String::from("Ana");
    user.email = String::from("Ana@gmail.com");
    user.sign_in_count = 3;
    user.active = true;

    println!("{:?}", user);

    user.add_by_five();

    println!("{:#?}", user);
    println!("{:#?}", User::BITS);


    let a = Some(5);
    let b: Option<i32> = None;
    println!("{:?}", a);
    let b = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    println!("{:?}", b);
}
#[derive(Debug)]
struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn add_by_five(&self) -> u64{
        self.sign_in_count + 5
    }
    pub const BITS: u32 = 32;
}

enum IPAddr{
    V4(String),
    V6(String),
}
