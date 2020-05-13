pub struct Solution {}

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut v: Vec<char> = (("0".to_string() + &num).chars()).collect();
        for i in 0..k as usize {
            for j in 0..v.len() - 1 {
                if v[j] > v[j + 1] {
                    v.remove(j);
                    break;
                }
            }
            if v.len() > num.len() - i {
                v.pop();
            }
        }
        while v.len() > 1 && v[0] == '0' {
            v.remove(0);
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
