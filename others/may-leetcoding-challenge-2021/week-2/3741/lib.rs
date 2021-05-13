pub struct Solution;

impl Solution {
    pub fn ambiguous_coordinates(s: String) -> Vec<String> {
        let s = s.chars().skip(1).take(s.len() - 2).collect::<Vec<_>>();
        let candidates = |v: &[char]| -> Vec<String> {
            (0..v.len())
                .filter_map(|i| {
                    let s: String = if i == 0 {
                        v.iter().collect()
                    } else {
                        v[..i]
                            .iter()
                            .chain(std::iter::once(&'.'))
                            .chain(v[i..].iter())
                            .collect()
                    };
                    if (s != "0" && s.starts_with('0') && !s.starts_with("0."))
                        || (s.contains('.') && s.ends_with('0'))
                    {
                        None
                    } else {
                        Some(s)
                    }
                })
                .collect()
        };
        let mut answer = Vec::new();
        for i in 1..s.len() {
            for x in &candidates(&s[..i]) {
                for y in &candidates(&s[i..]) {
                    answer.push(format!("({}, {})", x, y));
                }
            }
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret = Solution::ambiguous_coordinates(String::from("(123)"));
        ret.sort_unstable();
        assert_eq!(vec!["(1, 2.3)", "(1, 23)", "(1.2, 3)", "(12, 3)"], ret);
    }

    #[test]
    fn example_2() {
        let mut ret = Solution::ambiguous_coordinates(String::from("(00011)"));
        ret.sort_unstable();
        assert_eq!(vec!["(0, 0.011)", "(0.001, 1)"], ret);
    }

    #[test]
    fn example_3() {
        let mut ret = Solution::ambiguous_coordinates(String::from("(0123)"));
        ret.sort_unstable();
        assert_eq!(
            vec![
                "(0, 1.23)",
                "(0, 12.3)",
                "(0, 123)",
                "(0.1, 2.3)",
                "(0.1, 23)",
                "(0.12, 3)"
            ],
            ret
        );
    }

    #[test]
    fn example_4() {
        let mut ret = Solution::ambiguous_coordinates(String::from("(100)"));
        ret.sort_unstable();
        assert_eq!(vec!["(10, 0)"], ret);
    }
}
