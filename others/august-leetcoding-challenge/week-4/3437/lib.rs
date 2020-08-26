pub struct Solution {}

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..=n)
            .map(|i| match i {
                i if i % 15 == 0 => String::from("FizzBuzz"),
                i if i % 3 == 0 => String::from("Fizz"),
                i if i % 5 == 0 => String::from("Buzz"),
                _ => i.to_string(),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            vec![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz"
            ],
            Solution::fizz_buzz(15)
        );
    }
}
