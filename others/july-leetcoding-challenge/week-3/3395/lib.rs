pub struct Solution {}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut s: Vec<char> = Vec::with_capacity(std::cmp::max(a.len(), b.len()) + 1);
        let mut c: bool = false;
        let a = a.as_bytes();
        let b = b.as_bytes();
        for i in 0..std::cmp::max(a.len(), b.len()) {
            let mut num = if c { 1 } else { 0 };
            if i < a.len() && a[a.len() - 1 - i] == b'1' {
                num += 1;
            }
            if i < b.len() && b[b.len() - 1 - i] == b'1' {
                num += 1;
            }
            s.push(if num % 2 == 0 { '0' } else { '1' });
            c = num > 1;
        }
        if c {
            s.push('1');
        }
        s.reverse();
        s.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            "100",
            Solution::add_binary("11".to_string(), "1".to_string())
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            "10101",
            Solution::add_binary("1010".to_string(), "1011".to_string())
        );
    }
}
