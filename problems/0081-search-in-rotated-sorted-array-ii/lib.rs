pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        if nums.is_empty() {
            return false;
        }
        let (mut l, mut r) = (0, nums.len() - 1);
        while l <= r {
            let m = l + (r - l) / 2;
            let n = nums[m];
            if n == target {
                return true;
            }
            match n.cmp(&nums[r]) {
                std::cmp::Ordering::Greater => {
                    if target > n || target <= nums[r] {
                        l = m + 1;
                    } else if m > 0 {
                        r = m - 1;
                    } else {
                        break;
                    }
                }
                std::cmp::Ordering::Equal => {
                    if r > 0 {
                        r -= 1;
                    } else {
                        break;
                    }
                }
                std::cmp::Ordering::Less => {
                    if target <= nums[r] && target > n {
                        l = m + 1;
                    } else if m > 0 {
                        r = m - 1;
                    } else {
                        break;
                    }
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(true, Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0));
    }

    #[test]
    fn example_2() {
        assert_eq!(false, Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3));
    }
}
