pub struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut v = Vec::new();
        for d in path.split('/').filter(|s| !s.is_empty()) {
            match d {
                "." => {}
                ".." => {
                    v.pop();
                }
                _ => v.push(d),
            }
        }
        String::from("/") + &v.join("/")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!("/home", Solution::simplify_path(String::from("/home/")));
    }

    #[test]
    fn example_2() {
        assert_eq!("/", Solution::simplify_path(String::from("/../")));
    }

    #[test]
    fn example_3() {
        assert_eq!(
            "/home/foo",
            Solution::simplify_path(String::from("/home//foo/"))
        );
    }
}
