pub struct Solution {}

impl Solution {
    pub fn buddy_strings(a: String, b: String) -> bool {
        if a.len() != b.len() {
            return false;
        }
        let mut counts = [0; 26];
        let mut diff = Vec::new();
        let mut a: Vec<char> = a.chars().collect();
        for (i, (&ba, bb)) in a.iter().zip(b.chars()).enumerate() {
            counts[(ba as u8 - b'a') as usize] += 1;
            if ba != bb {
                diff.push(i);
            }
        }
        match diff.len() {
            0 => counts.iter().any(|&n| n > 1),
            2 => {
                a.swap(diff[0], diff[1]);
                a.iter().collect::<String>() == b
            }
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
