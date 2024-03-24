pub fn add(left: usize) -> usize {
    left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2);
        assert_eq!(result, 2);
    }
}
