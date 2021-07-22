pub struct Solution;

impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        let mins = nums
            .iter()
            .rev()
            .scan(i32::MAX, |state, &x| {
                *state = x.min(*state);
                Some(*state)
            })
            .collect::<Vec<_>>();
        nums.iter()
            .zip(mins.iter().rev().skip(1))
            .scan(0, |state, (&x, &min)| {
                *state = x.max(*state);
                if *state > min {
                    Some(*state)
                } else {
                    None
                }
            })
            .count() as i32
            + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(3, Solution::partition_disjoint(vec![5, 0, 3, 8, 6]));
    }

    #[test]
    fn example_2() {
        assert_eq!(4, Solution::partition_disjoint(vec![1, 1, 1, 0, 6, 12]));
    }
}
