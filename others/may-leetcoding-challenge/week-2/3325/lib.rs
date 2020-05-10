pub struct Solution {}

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut v: Vec<i32> = vec![0; n as usize];
        for t in trust.iter() {
            v[t[0] as usize - 1] -= 1;
            v[t[1] as usize - 1] += 1;
        }
        for (i, e) in v.iter().enumerate() {
            if *e == n - 1 {
                return i as i32 + 1;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(2, Solution::find_judge(2, vec![vec![1, 2]]));
    }

    #[test]
    fn example_2() {
        assert_eq!(3, Solution::find_judge(3, vec![vec![1, 3], vec![2, 3]]));
    }

    #[test]
    fn example_3() {
        assert_eq!(
            -1,
            Solution::find_judge(3, vec![vec![1, 3], vec![2, 3], vec![3, 1]])
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(-1, Solution::find_judge(3, vec![vec![1, 2], vec![2, 3]]));
    }

    #[test]
    fn example_5() {
        assert_eq!(
            3,
            Solution::find_judge(
                4,
                vec![vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![4, 3]]
            )
        );
    }
}
