pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_one(value: u64) -> u64 {
    value + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn it_fails() {
        let result = add(2, 2);
        assert_eq!(result, 5);
    }

    #[test]
    fn it_adds_one() {
        let result = add_one(1);
        assert_eq!(result, 2);
    }

    #[test]
    fn it_adds_two() {
        let result = add_one(2);
        assert_eq!(result, 3);
    }
}
