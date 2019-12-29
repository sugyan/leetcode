pub struct Solution {}

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut answer: Vec<String> = Vec::new();
        let mut v: Vec<String> = Vec::new();
        Solution::helper(&mut answer, &mut v, s.as_str(), 4);
        return answer;
    }
    fn helper(answer: &mut Vec<String>, v: &mut Vec<String>, s: &str, n: usize) {
        if n == 0 {
            if s.is_empty() {
                answer.push(v.join("."));
            }
            return;
        }
        for i in 0..3 {
            if let Some(t) = s.get(0..=i) {
                if t.parse::<u8>().is_ok() {
                    if t.starts_with('0') && t != "0" {
                        continue;
                    }
                    v.push(t.to_string());
                    Solution::helper(answer, v, s.get(i + 1..).unwrap(), n - 1);
                    v.pop();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret = Solution::restore_ip_addresses("25525511135".to_string());
        ret.sort();
        assert_eq!(vec!["255.255.11.135", "255.255.111.35"], ret);
    }
}
