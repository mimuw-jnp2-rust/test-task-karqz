fn test() -> bool {
    true
}

fn main() {
    assert!(test());
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(test());
    }
}
