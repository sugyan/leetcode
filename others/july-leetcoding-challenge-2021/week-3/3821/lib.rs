use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let len = dominoes.len();
        let bytes = dominoes.as_bytes();
        let mut v = vec![0; len];
        let mut f = None;
        for i in 0..len {
            f = match bytes[i] {
                b'R' => Some(0),
                b'.' => f.map(|j| j + 1),
                _ => None,
            };
            v[i] -= f.unwrap_or(len as i32);
        }
        for i in (0..len).rev() {
            f = match bytes[i] {
                b'L' => Some(0),
                b'.' => f.map(|j| j + 1),
                _ => None,
            };
            v[i] += f.unwrap_or(len as i32);
        }
        v.iter()
            .map(|f| match f.cmp(&0) {
                Ordering::Less => 'L',
                Ordering::Equal => '.',
                Ordering::Greater => 'R',
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!("RR.L", Solution::push_dominoes(String::from("RR.L")));
    }

    #[test]
    fn example_2() {
        assert_eq!(
            "LL.RR.LLRRLL..",
            Solution::push_dominoes(String::from(".L.R...LR..L.."))
        );
    }
}
