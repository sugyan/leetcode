use std::ops::RangeInclusive;

pub struct Solution;

impl Solution {
    pub fn superpalindromes_in_range(left: String, right: String) -> i32 {
        let mut answer = 0;
        if let (Ok(l), Ok(r)) = (left.parse::<u64>(), right.parse::<u64>()) {
            {
                let mut v = Vec::with_capacity(6);
                Self::backtrack(&mut answer, &mut v, &(l..=r), false);
            }
            {
                let mut v = Vec::with_capacity(6);
                Self::backtrack(&mut answer, &mut v, &(l..=r), true);
            }
        }
        answer
    }
    fn backtrack(answer: &mut i32, v: &mut Vec<u64>, range: &RangeInclusive<u64>, odd: bool) {
        if v.len() == 6 {
            let digits = if let Some(pos) = v.iter().position(|&d| d != 0) {
                &v[pos..]
            } else {
                &v
            };
            let mut n = 0;
            for &d in digits.iter() {
                n = n * 10 + d;
            }
            for &d in digits.iter().rev().skip(if odd { 1 } else { 0 }) {
                n = n * 10 + d;
            }
            if range.contains(&(n.saturating_mul(n))) {
                *answer += 1;
            }
        } else {
            let sum = v.iter().map(|d| d * d).sum::<u64>();
            for u in 0..=9 {
                if sum * 2 + u * u * if odd { 1 } else { 2 } < 10 {
                    v.push(u);
                    Self::backtrack(answer, v, range, odd);
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
