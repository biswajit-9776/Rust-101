fn main() {
    // Mutable and Immutable References
    let mut s = String::from("hello");
    func(&mut s);
    println!("{}", s);

    let mut s = String::from("hello");
    let r1 = &mut s;
    r1.push_str(", world!");
    println!("{:p}", r1);

    let m = 42;
    let r2 = &m;
    println!("{}", *r2);

    let s3 = String::from("hello");
    let r1 = &s3;
    println!("{}", r1);

    let s = "hellojediejfiejfj ejfiejfij";
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            println!("{}", i);
            break;
        }
    }
    println!("{}", bytes[0]);
}
fn func(some_string: &mut String) {
    some_string.push_str(", world!");
}
