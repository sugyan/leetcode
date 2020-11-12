use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut hm: HashMap<i32, usize> = HashMap::new();
        nums.iter()
            .for_each(|&num| *hm.entry(num).or_insert(0) += 1);
        let mut answer = Vec::new();
        let mut v = Vec::new();
        Solution::helper(&mut hm, &mut answer, &mut v);
        answer
    }
    fn helper(hm: &mut HashMap<i32, usize>, answer: &mut Vec<Vec<i32>>, v: &mut Vec<i32>) {
        let candidates: Vec<i32> = hm.iter().filter(|(_, &v)| v > 0).map(|(&k, _)| k).collect();
        if candidates.is_empty() {
            answer.push(v.clone());
        } else {
            for candidate in candidates.iter() {
                if let Some(val) = hm.get_mut(candidate) {
                    *val -= 1;
                }
                v.push(*candidate);
                Solution::helper(hm, answer, v);
                v.pop();
                if let Some(val) = hm.get_mut(candidate) {
                    *val += 1;
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
        let mut ret = Solution::permute_unique(vec![1, 1, 2]);
        ret.sort();
        assert_eq!(vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]], ret);
    }

    #[test]
    fn example_2() {
        let mut ret = Solution::permute_unique(vec![1, 2, 3]);
        ret.sort();
        assert_eq!(
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ],
            ret
        );
    }
}
