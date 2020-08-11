pub struct Solution {}

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut citations = citations;
        citations.sort();
        for i in (0..citations.len()).rev() {
            if citations[citations.len() - i - 1] > i as i32 {
                return i as i32 + 1;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(3, Solution::h_index(vec![3, 0, 6, 1, 5]));
    }
}
