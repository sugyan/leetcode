pub struct Solution;

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut sum = (0..k as usize).map(|i| (arr[i] - x).abs()).sum::<i32>();
        let (mut minsum, mut minidx) = (sum, 0);
        for i in 0..arr.len() - k as usize {
            sum += (arr[i + k as usize] - x).abs() - (arr[i] - x).abs();
            if sum < minsum {
                minsum = sum;
                minidx = i + 1;
            }
        }
        arr[minidx..minidx + k as usize].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            vec![1, 2, 3, 4],
            Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 4, 3)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![1, 2, 3, 4],
            Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 4, -1)
        );
    }
}
