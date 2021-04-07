pub struct Solution;

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let s = s.to_ascii_lowercase();
        let (first, second) = s.split_at(s.len() / 2);
        let is_vowel = |c: &char| match *c {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        };
        first.chars().filter(is_vowel).count() == second.chars().filter(is_vowel).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(true, Solution::halves_are_alike(String::from("book")));
    }

    #[test]
    fn example_2() {
        assert_eq!(false, Solution::halves_are_alike(String::from("textbook")));
    }

    #[test]
    fn example_3() {
        assert_eq!(
            false,
            Solution::halves_are_alike(String::from("MerryChristmas"))
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(true, Solution::halves_are_alike(String::from("AbCdEfGh")));
    }
}
