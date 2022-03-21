pub struct Solution;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut last = [0; 26];
        for (i, u) in s.bytes().enumerate() {
            last[(u - b'a') as usize] = i;
        }
        let mut answer = Vec::new();
        let mut p = (0, 0);
        for (i, u) in s.bytes().enumerate() {
            p.1 = p.1.max(last[(u - b'a') as usize]);
            if i == p.1 {
                answer.push((p.1 - p.0 + 1) as i32);
                p.0 = i + 1;
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

    #[test]
    fn example_2() {
        assert_eq!(
            vec![10],
            Solution::partition_labels(String::from("eccbbbbdec"))
        );
    }
}
