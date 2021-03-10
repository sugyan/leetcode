pub struct Solution;

const DICT: [(i32, &str); 13] = [
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut answer = String::new();
        let mut num = num;
        while num > 0 {
            if let Some(&(n, s)) = DICT.iter().find(|(n, _)| *n <= num) {
                answer += s;
                num -= n;
            }
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!("III", Solution::int_to_roman(3));
    }

    #[test]
    fn example_2() {
        assert_eq!("IV", Solution::int_to_roman(4));
    }

    #[test]
    fn example_3() {
        assert_eq!("IX", Solution::int_to_roman(9));
    }

    #[test]
    fn example_4() {
        assert_eq!("LVIII", Solution::int_to_roman(58));
    }

    #[test]
    fn example_5() {
        assert_eq!("MCMXCIV", Solution::int_to_roman(1994));
    }
}
