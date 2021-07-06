pub struct Solution;

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let mut counts = arr.iter().fold(vec![0; 100_001], |mut acc, &x| {
            acc[x as usize] += 1;
            acc
        });
        counts.sort_unstable();
        let mut sum = 0;
        for (i, &c) in counts.iter().rev().enumerate() {
            sum += c;
            if sum * 2 >= arr.len() {
                return i as i32 + 1;
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            2,
            Solution::min_set_size(vec![3, 3, 3, 3, 5, 5, 5, 2, 2, 7])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(1, Solution::min_set_size(vec![7, 7, 7, 7, 7, 7]));
    }

    #[test]
    fn example_3() {
        assert_eq!(1, Solution::min_set_size(vec![1, 9]));
    }

    #[test]
    fn example_4() {
        assert_eq!(1, Solution::min_set_size(vec![1000, 1000, 3, 7]));
    }

    #[test]
    fn example_5() {
        assert_eq!(
            5,
            Solution::min_set_size(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
        );
    }
}
