pub struct Solution;

impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<i32>) -> String {
        s.bytes()
            .rev()
            .zip(shifts.iter().rev().scan(0, |state, &x| {
                *state = (*state + x) % 26;
                Some(*state)
            }))
            .map(|(u, i)| (b'a' + (u - b'a' + i as u8) % 26) as char)
            .collect::<Vec<_>>()
            .iter()
            .rev()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            "rpl",
            Solution::shifting_letters(String::from("abc"), vec![3, 5, 9])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            "gfd",
            Solution::shifting_letters(String::from("aaa"), vec![1, 2, 3])
        );
    }
}
