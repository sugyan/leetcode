pub struct Solution;

impl Solution {
    pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .filter(|&num| num % 2 == 0)
            .zip(nums.iter().filter(|&num| num % 2 != 0))
            .flat_map(|(&even, &odd)| vec![even, odd].into_iter())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            vec![4, 5, 2, 7],
            Solution::sort_array_by_parity_ii(vec![4, 2, 5, 7])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(vec![2, 3], Solution::sort_array_by_parity_ii(vec![2, 3]));
    }
}
