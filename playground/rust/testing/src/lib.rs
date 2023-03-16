pub fn add(left: &usize, right: &usize) -> usize {
    let result = left + right;
    result
}

#[cfg(test)]
mod anything {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(&2, &2);
        assert_eq!(result, 4);
    }

    #[test]
    fn hello() {
        let result = add(&2, &4);
        assert_eq!(result, 6);
    }
}

