pub struct Solution {}

impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        let mut answer: Vec<i32> = (1..=9).collect();
        for _ in 0..n - 1 {
            let mut v: Vec<i32> = Vec::new();
            for m in answer.iter() {
                for d in if k == 0 { vec![0] } else { vec![-k, k] }.iter() {
                    let j = (m % 10) + d;
                    if j >= 0 && j < 10 {
                        v.push(m * 10 + j);
                    }
                }
            }
            answer = v;
        }
        if n == 1 {
            answer.push(0);
        }
        answer
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
