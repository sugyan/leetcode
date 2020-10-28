pub struct Solution {}

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut answer = Vec::new();
        let mut i = 0;
        while i < nums.len() {
            let min = nums[i];
            while i < nums.len() - 1 && nums[i] + 1 == nums[i + 1] {
                i += 1;
            }
            let mut s = min.to_string();
            if nums[i] != min {
                s += "->";
                s += &nums[i].to_string();
            }
            answer.push(s);
            i += 1;
        }
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
