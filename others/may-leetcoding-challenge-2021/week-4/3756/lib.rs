pub struct Solution;

impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        n.as_bytes().iter().map(|d| d - b'0').max().unwrap() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(3, Solution::min_partitions(String::from("32")));
    }

    #[test]
    fn example_2() {
        assert_eq!(8, Solution::min_partitions(String::from("82734")));
    }

    #[test]
    fn example_3() {
        assert_eq!(
            9,
            Solution::min_partitions(String::from("27346209830709182346"))
        );
    }
}
