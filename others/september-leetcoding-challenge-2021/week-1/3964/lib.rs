pub struct Solution;

impl Solution {
    pub fn orderly_queue(s: String, k: i32) -> String {
        if k == 1 {
            let ss = s.chars().chain(s.chars()).collect::<Vec<_>>();
            let mut v = ss.windows(s.len()).collect::<Vec<_>>();
            v.sort();
            v[0].iter().copied().collect()
        } else {
            let mut v = s.chars().collect::<Vec<_>>();
            v.sort();
            v.iter().collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!("acb", Solution::orderly_queue(String::from("cba"), 1));
    }

    #[test]
    fn example_2() {
        assert_eq!("aaabc", Solution::orderly_queue(String::from("baaca"), 3));
    }
}
