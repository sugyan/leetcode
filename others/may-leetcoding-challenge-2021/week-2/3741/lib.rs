pub struct Solution;

impl Solution {
    pub fn ambiguous_coordinates(s: String) -> Vec<String> {
        let v = s.as_bytes()[1..s.len() - 1]
            .iter()
            .map(|&u| u - b'0')
            .collect::<Vec<_>>();
        let mut answer = Vec::new();
        for i in 1..v.len() {
            let (mut vx, mut vy) = (Vec::new(), Vec::new());
            Self::dfs(&mut vx, &v[..i], String::new());
            Self::dfs(&mut vy, &v[i..], String::new());
            for x in &vx {
                for y in &vy {
                    answer.push(format!("({}, {})", x, y));
                }
            }
        }
        answer
    }
    fn dfs(ret: &mut Vec<String>, v: &[u8], s: String) {
        if let Some(&f) = v.first() {
            Self::dfs(ret, &v[1..], s.clone() + &f.to_string());
            if !(s.is_empty() || s.contains('.')) {
                Self::dfs(ret, &v[1..], s + "." + &f.to_string());
            }
        } else {
            if s != "0" && s.starts_with('0') && !s.starts_with("0.") {
                return;
            }
            if s.contains('.') && s.ends_with('0') {
                return;
            }
            ret.push(s);
        }
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
