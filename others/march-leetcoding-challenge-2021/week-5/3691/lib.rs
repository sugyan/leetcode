pub struct Solution;

impl Solution {
    pub fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
        let stamp = stamp.as_bytes();
        let mut target = target.into_bytes();
        let mut added = vec![false; target.len() - stamp.len() + 1];
        let mut answer = Vec::new();
        while target.iter().any(|&b| b != b'*') {
            let mut found = false;
            for i in 0..=target.len() - stamp.len() {
                if !added[i]
                    && (0..stamp.len()).all(|j| target[i + j] == b'*' || target[i + j] == stamp[j])
                {
                    answer.push(i as i32);
                    added[i] = true;
                    found = true;
                    (0..stamp.len()).for_each(|j| target[i + j] = b'*');
                }
            }
            if !found {
                return Vec::new();
            }
        }
        answer.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            vec![0, 2],
            Solution::moves_to_stamp(String::from("abc"), String::from("ababc"))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![3, 0, 1],
            Solution::moves_to_stamp(String::from("abca"), String::from("aabcaca"))
        );
    }
}
