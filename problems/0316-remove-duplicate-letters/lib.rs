pub struct Solution;

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut counts = [0; 26];
        for b in s.bytes() {
            counts[(b - b'a') as usize] += 1;
        }
        let mut v = Vec::with_capacity(26);
        for b in s.bytes() {
            counts[(b - b'a') as usize] -= 1;
            if v.contains(&b) {
                continue;
            }
            while let Some(&last) = v.last() {
                if b < last as u8 && counts[(last - b'a') as usize] > 0 {
                    v.pop();
                } else {
                    break;
                }
            }
            v.push(b)
        }
        v.iter().map(|&b| b as char).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            "abc",
            Solution::remove_duplicate_letters(String::from("bcabc"))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            "acdb",
            Solution::remove_duplicate_letters(String::from("cbacdcbc"))
        );
    }
}
