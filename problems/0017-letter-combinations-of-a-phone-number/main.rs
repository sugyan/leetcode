pub struct Solution {}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let mut m = std::collections::HashMap::new();
        m.insert('2', vec!['a', 'b', 'c']);
        m.insert('3', vec!['d', 'e', 'f']);
        m.insert('4', vec!['g', 'h', 'i']);
        m.insert('5', vec!['j', 'k', 'l']);
        m.insert('6', vec!['m', 'n', 'o']);
        m.insert('7', vec!['p', 'q', 'r', 's']);
        m.insert('8', vec!['t', 'u', 'v']);
        m.insert('9', vec!['w', 'x', 'y', 'z']);
        let mut q = std::collections::VecDeque::new();
        q.push_back("".to_string());
        for (i, c) in (0..).zip(digits.chars()) {
            while let Some(front) = q.front() {
                if front.len() == i {
                    if let Some(s) = q.pop_front() {
                        if let Some(v) = m.get(&c) {
                            for &a in v {
                                let mut s = s.clone();
                                s.push(a);
                                q.push_back(s);
                            }
                        }
                    }
                } else {
                    break;
                }
            }
        }
        return Vec::from(q);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret = Solution::letter_combinations("23".to_string());
        ret.sort();
        assert_eq!(
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"],
            ret,
        );
    }
}
