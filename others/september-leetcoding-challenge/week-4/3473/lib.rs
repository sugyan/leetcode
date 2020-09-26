pub struct Solution {}

impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let mut answer = 0;
        let mut until = -1;
        for &s in time_series.iter() {
            answer += duration;
            if until >= s {
                answer -= until - s + 1;
            }
            until = s + duration - 1;
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(4, Solution::find_poisoned_duration(vec![1, 4], 2));
    }

    #[test]
    fn example_2() {
        assert_eq!(3, Solution::find_poisoned_duration(vec![1, 2], 2));
    }
}
