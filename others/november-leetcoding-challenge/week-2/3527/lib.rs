use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
        let mut hm: HashMap<i32, usize> = HashMap::new();
        let dist = |p: &Vec<i32>, q: &Vec<i32>| -> i32 {
            let x = q[0] - p[0];
            let y = q[1] - p[1];
            x * x + y * y
        };
        [
            dist(&p1, &p2),
            dist(&p1, &p3),
            dist(&p1, &p4),
            dist(&p2, &p3),
            dist(&p2, &p4),
            dist(&p3, &p4),
        ]
        .iter()
        .for_each(|&d| *hm.entry(d).or_insert(0) += 1);
        let values: Vec<usize> = hm.iter().map(|(_, &v)| v).collect();
        values == vec![2, 4] || values == vec![4, 2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            true,
            Solution::valid_square(vec![0, 0], vec![1, 1], vec![1, 0], vec![0, 1])
        );
    }
}
