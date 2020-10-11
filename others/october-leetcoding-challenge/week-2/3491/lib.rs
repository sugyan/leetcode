pub struct Solution {}

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut counts: [usize; 26] = [0; 26];
        for &b in s.as_bytes() {
            counts[(b - b'a') as usize] += 1;
        }
        let mut v: Vec<char> = Vec::with_capacity(26);
        let mut exists: [bool; 26] = [false; 26];
        for &b in s.as_bytes() {
            let i = (b - b'a') as usize;
            counts[i] -= 1;
            if exists[i] {
                continue;
            }
            while let Some(&last) = v.last() {
                let j = (last as u8 - b'a') as usize;
                if b < last as u8 && counts[j] > 0 {
                    exists[j] = false;
                    v.pop();
                } else {
                    break;
                }
            }
            v.push(b as char);
            exists[i] = true;
        }
        v.iter().collect()
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
