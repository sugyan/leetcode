pub struct Solution;

impl Solution {
    pub fn superpalindromes_in_range(left: String, right: String) -> i32 {
        let mut answer = 0;
        if let (Ok(l), Ok(r)) = (left.parse::<u64>(), right.parse::<u64>()) {
            for i in 1..=right.len() {
                let mut candidates = Vec::new();
                let mut v = Vec::new();
                Self::backtrack(&mut candidates, &mut v, i, 0);
                for &c in &candidates {
                    if c > r {
                        return answer;
                    }
                    if l <= c && c <= r {
                        answer += 1;
                    }
                }
            }
        }
        answer
    }
    fn backtrack(candidates: &mut Vec<u64>, v: &mut Vec<u8>, len: usize, sum: u8) {
        if v.len() == (len + 1) / 2 {
            println!("{:?} ({})", v, len);
            let candidate = v.iter().rev().skip(if len % 2 != 0 { 1 } else { 0 }).fold(
                v.iter().fold(0, |acc, &u| acc * 10 + u as u64),
                |acc, &u| acc * 10 + u as u64,
            );
            candidates.push(candidate * candidate);
            return;
        }
        for u in 0..=9 {
            if v.is_empty() && u == 0 {
                continue;
            }
            if sum * 2 + u * u * if len % 2 == 0 { 2 } else { 1 } < 10 {
                v.push(u);
                Self::backtrack(candidates, v, len, sum + u * u);
                v.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            4,
            Solution::superpalindromes_in_range(String::from("4"), String::from("1000"))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            1,
            Solution::superpalindromes_in_range(String::from("1"), String::from("2"))
        );
    }
}
