pub struct Solution {}

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut v: Vec<char> = Vec::with_capacity(num.len());
        let mut k = k;
        for c in num.chars() {
            while k > 0 {
                if let Some(l) = v.last() {
                    if *l > c {
                        v.pop();
                        k -= 1;
                        continue;
                    }
                }
                break;
            }
            if !v.is_empty() || c != '0' {
                v.push(c);
            }
        }
        while k > 0 {
            v.pop();
            k -= 1;
        }
        if v.is_empty() {
            v.push('0');
        }
        v.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!("1219", Solution::remove_kdigits("1432219".to_string(), 3));
    }

    #[test]
    fn example_2() {
        assert_eq!("200", Solution::remove_kdigits("10200".to_string(), 1));
    }

    #[test]
    fn example_3() {
        assert_eq!("0", Solution::remove_kdigits("10".to_string(), 2));
    }
}
