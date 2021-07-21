pub struct Solution;

impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let len = dominoes.len();
        let mut chars = dominoes.chars().collect::<Vec<_>>();
        let mut v = vec![(None, None); len];
        let (mut l, mut r) = (None, None);
        for i in 0..len {
            r = match chars[i] {
                'R' => Some(0),
                '.' => r.map(|j| j + 1),
                _ => None,
            };
            v[i].1 = r;
            l = match chars[len - 1 - i] {
                'L' => Some(0),
                '.' => l.map(|j| j + 1),
                _ => None,
            };
            v[len - 1 - i].0 = l;
        }
        for (i, c) in chars.iter_mut().enumerate() {
            if *c == '.' {
                *c = match v[i] {
                    (Some(_), None) => 'L',
                    (None, Some(_)) => 'R',
                    (Some(l), Some(r)) if l != r => {
                        if l > r {
                            'R'
                        } else {
                            'L'
                        }
                    }
                    _ => *c,
                }
            }
        }
        chars.iter().collect()
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
