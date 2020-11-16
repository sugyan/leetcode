pub struct Solution {}

impl Solution {
    pub fn longest_mountain(a: Vec<i32>) -> i32 {
        let mut answer = 0;
        let mut i = 1;
        while i < a.len() {
            let (mut inc, mut dec) = (0, 0);
            while i < a.len() && a[i - 1] < a[i] {
                i += 1;
                inc += 1;
            }
            while i < a.len() && a[i - 1] > a[i] {
                i += 1;
                dec += 1;
            }
            if inc > 0 && dec > 0 {
                answer = std::cmp::max(answer, inc + dec + 1);
            }
            while i < a.len() && a[i - 1] == a[i] {
                i += 1;
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
        assert_eq!(5, Solution::longest_mountain(vec![2, 1, 4, 7, 3, 2, 5]));
    }

    #[test]
    fn example_2() {
        assert_eq!(0, Solution::longest_mountain(vec![2, 2, 2]));
    }
}
