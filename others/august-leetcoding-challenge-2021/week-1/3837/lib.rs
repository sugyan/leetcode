use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut counts = vec![0; 21];
        for &num in &nums {
            counts[(num + 10) as usize] += 1;
        }
        let mut vd = VecDeque::new();
        vd.push_back(Vec::new());
        for (i, &c) in counts.iter().enumerate() {
            for _ in 0..vd.len() {
                if let Some(mut front) = vd.pop_front() {
                    vd.push_back(front.clone());
                    for _ in 0..c {
                        front.push(i as i32 - 10);
                        vd.push_back(front.clone());
                    }
                }
            }
        }
        vd.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret = Solution::subsets_with_dup(vec![1, 2, 2]);
        ret.sort();
        assert_eq!(
            vec![
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 2, 2],
                vec![2],
                vec![2, 2]
            ],
            ret
        );
    }

    #[test]
    fn example_2() {
        let mut ret = Solution::subsets_with_dup(vec![0]);
        ret.sort();
        assert_eq!(vec![vec![], vec![0]], ret);
    }
}
