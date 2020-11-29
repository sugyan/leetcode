use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let mut arr = arr;
        let mut vd: VecDeque<i32> = VecDeque::with_capacity(arr.len());
        vd.push_back(start);
        while let Some(front) = vd.pop_front() {
            match arr[front as usize].cmp(&0) {
                std::cmp::Ordering::Less => continue,
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Greater => {
                    if front - arr[front as usize] >= 0 {
                        vd.push_back(front - arr[front as usize]);
                    }
                    if front + arr[front as usize] < arr.len() as i32 {
                        vd.push_back(front + arr[front as usize]);
                    }
                    arr[front as usize] = -1;
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
