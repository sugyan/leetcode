pub struct Solution;

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        (1..=target).fold(vec![1], |mut v, x| {
            v.push(
                nums.iter()
                    .filter_map(|&n| {
                        if n <= x {
                            Some(v[(x - n) as usize])
                        } else {
                            None
                        }
                    })
                    .sum(),
            );
            v
        })[target as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(7, Solution::combination_sum4(vec![1, 2, 3], 4));
    }

    #[test]
    fn example_2() {
        assert_eq!(0, Solution::combination_sum4(vec![9], 3));
    }
}
