pub struct Solution {}

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut sum = 0;
        let (mut idx, mut min) = (0, 0);
        for i in 0..gas.len() {
            sum += gas[i] - cost[i];
            if sum < min {
                min = sum;
                idx = i + 1;
            }
        }
        if sum < 0 {
            -1
        } else {
            idx as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            3,
            Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            -1,
            Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3])
        );
    }
}
