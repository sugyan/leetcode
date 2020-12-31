use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut answer = 0;
        let mut hm = HashMap::new();
        for (i, &height) in heights.iter().enumerate() {
            hm.insert(
                height,
                if let Some(min) = hm
                    .iter()
                    .filter(|(&k, _)| k >= height)
                    .map(|(_, &v)| v)
                    .min()
                {
                    min
                } else {
                    i
                },
            );
            hm.retain(|&k, _| k <= height);
            for (&k, &v) in hm.iter() {
                answer = std::cmp::max(answer, (i + 1 - v) as i32 * k);
            }
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(10, Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]));
    }
}
