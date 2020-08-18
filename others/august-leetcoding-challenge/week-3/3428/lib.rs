use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        let mut vd: VecDeque<i32> = (1..=9).collect();
        let border = (0..n - 1).fold(1, |acc, _| acc * 10);
        while let Some(&front) = vd.front() {
            if front >= border {
                break;
            }
            vd.pop_front();
            for d in if k == 0 { vec![0] } else { vec![-k, k] }.iter() {
                let j = (front % 10) + d;
                if j >= 0 && j < 10 {
                    vd.push_back(front * 10 + j);
                }
            }
        }
        if n == 1 {
            vd.push_front(0);
        }
        Vec::from(vd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret = Solution::nums_same_consec_diff(3, 7);
        ret.sort();
        assert_eq!(vec![181, 292, 707, 818, 929], ret);
    }

    #[test]
    fn example_2() {
        let mut ret = Solution::nums_same_consec_diff(2, 1);
        ret.sort();
        assert_eq!(
            vec![10, 12, 21, 23, 32, 34, 43, 45, 54, 56, 65, 67, 76, 78, 87, 89, 98],
            ret
        );
    }
}
