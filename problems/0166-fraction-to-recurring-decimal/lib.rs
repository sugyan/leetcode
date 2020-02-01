use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let mut n = numerator as i64;
        let mut d = denominator as i64;
        let mut answer = String::new();
        if n != 0 && (n < 0) != (d < 0) {
            answer.push('-');
        }
        n = n.abs();
        d = d.abs();
        answer += &(n / d).to_string();
        n %= d;
        if n > 0 {
            let mut hm: HashMap<i64, usize> = HashMap::new();
            hm.insert(n, 0);
            let mut s: String = String::new();
            while n != 0 {
                n *= 10;
                s.push((b'0' + (n / d) as u8) as char);
                n %= d;
                if hm.contains_key(&n) {
                    break;
                }
                hm.insert(n, s.len());
            }
            if n != 0 {
                s.insert(hm[&n], '(');
                s.push(')');
            }
            answer.push('.');
            answer += &s;
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!("0.5", Solution::fraction_to_decimal(1, 2));
    }

    #[test]
    fn example_2() {
        assert_eq!("2", Solution::fraction_to_decimal(2, 1));
    }

    #[test]
    fn example_3() {
        assert_eq!("0.(6)", Solution::fraction_to_decimal(2, 3));
    }
}
