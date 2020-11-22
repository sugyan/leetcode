use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let dict: [&str; 26] = [
            ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..",
            "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-",
            "-.--", "--..",
        ];
        let mut hs: HashSet<String> = HashSet::new();
        for word in words.iter() {
            let mut s = String::new();
            for &b in word.as_bytes() {
                s += dict[(b - b'a') as usize];
            }
            hs.insert(s);
        }
        hs.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            2,
            Solution::unique_morse_representations(vec![
                String::from("gin"),
                String::from("zen"),
                String::from("gig"),
                String::from("msg")
            ])
        );
    }
}
