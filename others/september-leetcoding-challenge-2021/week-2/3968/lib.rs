pub struct Solution;

impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<i32>) -> String {
        s.bytes()
            .zip(&shifts)
            .scan(
                shifts.iter().fold(0, |acc, &x| (acc + x) % 26),
                |state, (u, &shift)| {
                    let s = *state as u8;
                    *state = (*state - shift).rem_euclid(26);
                    Some((b'a' + (u - b'a' + s) % 26) as char)
                },
            )
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
