pub struct Solution {}

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut v: Vec<i32> = Vec::with_capacity(gas.len());
        v.push(gas[0] - cost[0]);
        let (mut idx, mut min) = (0, v[0]);
        for i in 1..gas.len() {
            v.push(v[i - 1] + gas[i] - cost[i]);
            if v[i] < min {
                idx = i;
                min = v[i];
            }
        }
        if v[v.len() - 1] < 0 {
            -1
        } else {
            ((idx + 1) % v.len()) as i32
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
