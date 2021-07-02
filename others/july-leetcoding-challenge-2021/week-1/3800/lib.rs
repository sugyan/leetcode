pub struct Solution;

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let (mut lo, mut hi) = (0, arr.len() - k as usize);
        while lo < hi {
            let m = (lo + hi) / 2;
            if x - arr[m] > arr[m + k as usize] - x {
                lo = m + 1
            } else {
                hi = m;
            }
        }
        arr[lo..lo + k as usize].to_vec()
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
