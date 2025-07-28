fn main() {
    let x = String::from("abcd");
    let y ="xyz";
    let reuslt = longest(x.as_str(), y);
    println!("{}", reuslt);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let f = novel.split('.').next().expect("Could not find a '.'");
    let result = ImportantExcerpt{part: f};
    println!("{}", result.part);
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }
    else {
        y
    }
}
struct ImportantExcerpt <'a> {
    part: &'a str,
}

fn print_10(a : i32) -> i32 {
    println!("The value is {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_10() {
        let result = print_10(4);
        assert_eq!(10, result);
    }
    #[test]
    fn test_print_10_fail() {
        let result = print_10(4);
        assert_eq!(11, result); // This will fail
    }
}