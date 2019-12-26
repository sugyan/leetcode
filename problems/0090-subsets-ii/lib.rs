pub struct Solution {}

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut hs: std::collections::HashSet<Vec<i32>> = std::collections::HashSet::new();
        hs.insert(vec![]);
        let mut nums = nums;
        nums.sort();
        for num in nums {
            let entries: Vec<Vec<i32>> = hs.iter().map(|v| v.clone()).collect();
            for v in entries {
                if let Some(last) = v.last() {
                    if *last > num {
                        continue;
                    }
                }
                let mut v = v.clone();
                v.push(num);
                hs.insert(v);
            }
        }
        return hs.into_iter().collect();
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
}
