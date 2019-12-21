#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn part1_test() {
        assert_eq!(1215, part1(&input(None)));
    }

    #[test]
    fn input_test() {
        let s = String::from("1234567890");
        let v: Vec<u32> = s.chars().map(|c| c.to_digit(10).unwrap()).collect();
        assert_eq!(v, input(Some(&s)));
    }

}
