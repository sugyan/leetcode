pub struct Solution {}

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, citations.len());
        while l < r {
            let m = l + (r - l) / 2;
            if citations[citations.len() - 1 - m] > m as i32 {
                l = m + 1
            } else {
                r = m
            }
        }
        l as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(3, Solution::h_index(vec![0, 1, 3, 5, 6]));
    }
}
