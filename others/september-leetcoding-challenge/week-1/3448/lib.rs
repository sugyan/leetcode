pub struct Solution {}

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut answer = Vec::new();
        let mut last = [0; 26];
        for (i, c) in s.as_bytes().iter().enumerate() {
            last[(c - b'a') as usize] = i;
        }
        let (mut l, mut r) = (0, 0);
        for (i, c) in s.as_bytes().iter().enumerate() {
            r = std::cmp::max(r, last[(c - b'a') as usize]);
            if i == r {
                answer.push((r - l + 1) as i32);
                l = i + 1;
            }
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            vec![9, 7, 8],
            Solution::partition_labels(String::from("ababcbacadefegdehijhklij"))
        );
    }
}
