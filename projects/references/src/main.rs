fn main() {
    // Mutable and Immutable References
    let mut s0 = String::from("hello");
    func(&mut s0);
    println!("{}", s0);

    // Cannot have mutable borrows more than once in a lifetime
    // i.e, we can't use the reference and variable in same scope
    let mut s1 = String::from("hello1");
    let r1 = &mut s1; // here borrowed as mutable
    r1.push_str(", world!"); // here the lifetime of the mutable borrow ends
    s1.clear(); // here borrowed as mutable
    // r1.push_str("efe"); // here first borrow is used again so error occurs

    // This code still runs as lifetime of word finishes before s1.clear() as it's not used anywhere else
    let mut s1 = String::from("hello world");
    let word = first_word(&s1);
    assert_eq!(word.chars().nth(0), Some('h'));
    s1.clear();

    let s1 = String::from("hello1");
    let r1 = &s1[..]; // here borrowed as mutable
    println!("{}", r1); // here mutable borrow is used once and not after so Rust thinks its lifetime is over
    println!("{}", s1);

    let mut s1 = String::from("hello1");
    let r1 = &mut s1;
    r1.push_str(", world!");
    println!("{}", r1); // here borrowed as mutable
    println!("{}", s1); // here borrowed as immutable so okay as Rust thinks r1's lifetime is over
    // println!("{}", r1); // here mutable borrow used again which Rust thought has its lifetime over

    let s2 = 42;
    let r2 = &s2;
    println!("{}", *r2);

    let s3 = String::from("hello");
    let r3 = &s3;
    println!("{}", r3);

    let s4 = String::from("hello4");
    let mut r4 = s4;
    r4.push_str(", world!");
    println!("{}", r4);

    let s5 = String::from("hello5");
    let mut r5 = s5;
    r5.push_str(", world!");
    println!("{}", r5);

    let mut s6 = String::from("hello6");
    let r6 = &mut s6;
    r6.push_str(", world!");
    println!("{}", r6);

    let s7 = String::from("hello7");
    let r7 = &s7;
    println!("{}", r7);
    println!("{}", s7);

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
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
