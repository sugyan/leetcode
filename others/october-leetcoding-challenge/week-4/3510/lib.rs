pub struct Solution {}

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut answer = Vec::new();
        if nums.is_empty() {
            return answer;
        }
        let mut min = nums[0];
        let mut s = min.to_string();
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] + 1 {
                if nums[i - 1] != min {
                    s += "->";
                    s += &nums[i - 1].to_string();
                }
                answer.push(s.clone());
                min = nums[i];
                s = min.to_string();
            }
        }
        if let Some(&last) = nums.last() {
            if last != min {
                s += "->";
                s += &last.to_string();
            }
        }
        answer.push(s);
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            vec!["0->2", "4->5", "7"],
            Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec!["0", "2->4", "6", "8->9"],
            Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9])
        );
    }

    #[test]
    fn example_3() {
        let v: Vec<String> = vec![];
        assert_eq!(v, Solution::summary_ranges(vec![]));
    }

    #[test]
    fn example_4() {
        assert_eq!(vec!["-1"], Solution::summary_ranges(vec![-1]));
    }

    #[test]
    fn example_5() {
        assert_eq!(vec!["0"], Solution::summary_ranges(vec![0]));
    }
}
