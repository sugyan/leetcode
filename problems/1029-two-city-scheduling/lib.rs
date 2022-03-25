pub struct Solution;

impl Solution {
    pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
        let mut costs = costs;
        costs.sort_by_cached_key(|c| c[0] - c[1]);
        (0..costs.len() / 2)
            .map(|i| costs[i][0] + costs[i + costs.len() / 2][1])
            .sum()
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

    #[test]
    fn example_2() {
        assert_eq!(
            1859,
            Solution::two_city_sched_cost(vec![
                vec![259, 770],
                vec![448, 54],
                vec![926, 667],
                vec![184, 139],
                vec![840, 118],
                vec![577, 469]
            ])
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            3086,
            Solution::two_city_sched_cost(vec![
                vec![515, 563],
                vec![451, 713],
                vec![537, 709],
                vec![343, 819],
                vec![855, 779],
                vec![457, 60],
                vec![650, 359],
                vec![631, 42]
            ])
        );
    }
}
