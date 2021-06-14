use std::cmp::Reverse;

pub struct Solution;

impl Solution {
    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        let mut box_types = box_types;
        let mut truck_size = truck_size;
        box_types.sort_unstable_by_key(|bt| Reverse(bt[1]));
        let mut answer = 0;
        let mut i = 0;
        while i < box_types.len() && truck_size > 0 {
            let num = box_types[i][0].min(truck_size);
            answer += num * box_types[i][1];
            truck_size -= num;
            i += 1;
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            8,
            Solution::maximum_units(vec![vec![1, 3], vec![2, 2], vec![3, 1]], 4)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            91,
            Solution::maximum_units(vec![vec![5, 10], vec![2, 5], vec![4, 7], vec![3, 9]], 10)
        );
    }
}
