use std::str::Bytes;

#[derive(PartialEq)]
enum Op {
    Add,
    Sub,
}

pub struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        Solution::helper(&mut s.bytes())
    }
    fn helper(iter: &mut Bytes) -> i32 {
        let mut ret = 0;
        let mut op = Op::Add;
        let mut n = 0;
        while let Some(u) = iter.next() {
            match u {
                b'+' | b'-' => {
                    ret += if op == Op::Add { n } else { -n };
                    op = if u == b'+' { Op::Add } else { Op::Sub };
                    n = 0;
                }
                b'0'..=b'9' => n = n * 10 + (u - b'0') as i32,
                b'(' => n = Self::helper(iter),
                b')' => break,
                _ => {}
            }
        }
        ret + if op == Op::Add { n } else { -n }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(2, Solution::calculate(String::from("1 + 1")));
    }

    #[test]
    fn example_2() {
        assert_eq!(3, Solution::calculate(String::from(" 2-1 + 2 ")));
    }

    #[test]
    fn example_3() {
        assert_eq!(23, Solution::calculate(String::from("(1+(4+5+2)-3)+(6+8)")));
    }
}
