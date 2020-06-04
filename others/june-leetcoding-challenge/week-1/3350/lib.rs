pub struct Solution {}

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let l = s.len();
        for i in 0..s.len() / 2 {
            s.swap(i, l - i - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut s: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&mut s);
        assert_eq!(vec!['o', 'l', 'l', 'e', 'h'], s);
    }

    #[test]
    fn example_2() {
        let mut s: Vec<char> = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        Solution::reverse_string(&mut s);
        assert_eq!(vec!['h', 'a', 'n', 'n', 'a', 'H'], s);
    }
}
