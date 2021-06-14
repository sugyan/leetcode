pub struct Solution;

impl Solution {
    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        let mut box_types = box_types;
        box_types.sort_unstable_by_key(|bt| bt[1]);
        box_types
            .iter()
            .rev()
            .fold((0, truck_size), |(units, remaining), box_type| {
                let num = box_type[0].min(remaining);
                (units + num * box_type[1], remaining - num)
            })
            .0
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
