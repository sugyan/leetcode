pub struct Solution {}

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut d: [usize; 256] = [0; 256];
        for c in magazine.chars().map(|c| (c as u8) as usize) {
            d[c] += 1;
        }
        for c in ransom_note.chars().map(|c| (c as u8) as usize) {
            if d[c] == 0 {
                return false;
            }
            d[c] -= 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            false,
            Solution::can_construct("a".to_string(), "b".to_string())
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            false,
            Solution::can_construct("aa".to_string(), "ab".to_string())
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            true,
            Solution::can_construct("aa".to_string(), "aab".to_string())
        );
    }
}
