use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
        let mut bh: BinaryHeap<i32> = BinaryHeap::with_capacity(costs.len());
        let mut answer = 0;
        for cost in costs.iter() {
            answer += cost[0];
            bh.push(cost[0] - cost[1]);
        }
        for _ in 0..costs.len() / 2 {
            if let Some(min) = bh.pop() {
                answer -= min;
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
        assert_eq!(
            110,
            Solution::two_city_sched_cost(vec![
                vec![10, 20],
                vec![30, 200],
                vec![400, 50],
                vec![30, 20]
            ])
        );
    }
}
