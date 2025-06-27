fn main() {
    let mut user = User {
        name: String::from("Patrick"),
        email: String::from("patrick@gmail.com"),
        sign_in_count: 1,
        active: true,
    };
    user.name = String::from("Ana");
    println!("{:?}", user);
    println!("{:#?}", user);
}
#[derive(Debug)]
struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn addByFive(&self) -> u64{
        self.sign_in_count + 5
    }
}
