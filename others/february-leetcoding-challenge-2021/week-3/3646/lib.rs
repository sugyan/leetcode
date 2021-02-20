pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut answer = 0;
        let mut prev = None;
        for c in s.chars() {
            answer += match c {
                'I' => 1,
                'V' if prev == Some('I') => 3,
                'V' => 5,
                'X' if prev == Some('I') => 8,
                'X' => 10,
                'L' if prev == Some('X') => 30,
                'L' => 50,
                'C' if prev == Some('X') => 80,
                'C' => 100,
                'D' if prev == Some('C') => 300,
                'D' => 500,
                'M' if prev == Some('C') => 800,
                'M' => 1000,
                _ => unreachable!(),
            };
            prev = Some(c);
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(3, Solution::roman_to_int(String::from("III")));
    }

    #[test]
    fn example_2() {
        assert_eq!(4, Solution::roman_to_int(String::from("IV")));
    }

    #[test]
    fn example_3() {
        assert_eq!(9, Solution::roman_to_int(String::from("IX")));
    }

    #[test]
    fn example_4() {
        assert_eq!(58, Solution::roman_to_int(String::from("LVIII")));
    }

    #[test]
    fn example_5() {
        assert_eq!(1994, Solution::roman_to_int(String::from("MCMXCIV")));
    }
}
