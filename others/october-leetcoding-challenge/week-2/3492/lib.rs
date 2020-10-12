pub struct Solution {}

impl Solution {
    pub fn buddy_strings(a: String, b: String) -> bool {
        if a.len() != b.len() {
            return false;
        }
        let diff: Vec<(char, char)> = a
            .chars()
            .zip(b.chars())
            .filter(|(ca, cb)| ca != cb)
            .collect();
        match diff.len() {
            0 => {
                let mut counts = [0; 26];
                for b in a.as_bytes().iter() {
                    counts[(b - b'a') as usize] += 1;
                    if counts[(b - b'a') as usize] > 1 {
                        return true;
                    }
                }
                false
            }
            2 => diff[0].0 == diff[1].1 && diff[0].1 == diff[1].0,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            true,
            Solution::buddy_strings(String::from("ab"), String::from("ba"))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            false,
            Solution::buddy_strings(String::from("ab"), String::from("ab"))
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            true,
            Solution::buddy_strings(String::from("aa"), String::from("aa"))
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            true,
            Solution::buddy_strings(String::from("aaaaaaabc"), String::from("aaaaaaacb"))
        );
    }

    #[test]
    fn example_5() {
        assert_eq!(
            false,
            Solution::buddy_strings(String::from(""), String::from("aa"))
        );
    }
}
