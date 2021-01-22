pub struct Solution;

impl Solution {
    pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut stack = Vec::with_capacity(k as usize);
        for (i, &num) in nums.iter().enumerate() {
            while let Some(&last) = stack.last() {
                if num < last && stack.len() + nums.len() - i > k as usize {
                    stack.pop();
                } else {
                    break;
                }
            }
            if stack.len() < k as usize {
                stack.push(num);
            }
        }
        stack
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(vec![2, 6], Solution::most_competitive(vec![3, 5, 2, 6], 2));
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![2, 3, 3, 4],
            Solution::most_competitive(vec![2, 4, 3, 3, 5, 4, 9, 6], 4)
        );
    }

    #[test]
    fn re_1() {
        assert_eq!(
            vec![8, 80, 2],
            Solution::most_competitive(vec![71, 18, 52, 29, 55, 73, 24, 42, 66, 8, 80, 2], 3)
        );
    }

    #[test]
    fn wa_1() {
        assert_eq!(
            vec![
                10, 23, 61, 62, 34, 41, 80, 25, 91, 43, 4, 75, 65, 13, 37, 41, 46, 90, 55, 8, 85,
                61, 95, 71
            ],
            Solution::most_competitive(
                vec![
                    84, 10, 71, 23, 66, 61, 62, 64, 34, 41, 80, 25, 91, 43, 4, 75, 65, 13, 37, 41,
                    46, 90, 55, 8, 85, 61, 95, 71
                ],
                24
            )
        );
    }
}
