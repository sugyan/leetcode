use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let mut hs: HashSet<i32> = HashSet::new();
        let mut stack: Vec<i32> = Vec::new();
        stack.push(start);
        while let Some(last) = stack.pop() {
            if arr[last as usize] == 0 {
                return true;
            }
            for &i in [last + arr[last as usize], last - arr[last as usize]].iter() {
                if i >= 0 && (i as usize) < arr.len() && !hs.contains(&i) {
                    hs.insert(i);
                    stack.push(i);
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(true, Solution::can_reach(vec![4, 2, 3, 0, 3, 1, 2], 5));
    }

    #[test]
    fn example_2() {
        assert_eq!(true, Solution::can_reach(vec![4, 2, 3, 0, 3, 1, 2], 0));
    }

    #[test]
    fn example_3() {
        assert_eq!(false, Solution::can_reach(vec![3, 0, 2, 1, 2], 2));
    }
}
