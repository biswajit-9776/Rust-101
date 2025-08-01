pub fn add_two(x: u32) -> u32 {
    x + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add_two(3), 5);
    }
}
