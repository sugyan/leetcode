use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        let mut bh = BinaryHeap::new();
        let mut tank = start_fuel;
        let mut answer = 0;
        for station in stations.iter().chain(&vec![vec![target, 0]]) {
            while tank < station[0] {
                if let Some(max) = bh.pop() {
                    tank += max;
                    answer += 1;
                } else {
                    return -1;
                }
            }
            bh.push(station[1]);
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(0, Solution::min_refuel_stops(1, 1, vec![]));
    }

    #[test]
    fn example_2() {
        assert_eq!(-1, Solution::min_refuel_stops(100, 1, vec![vec![10, 100]]));
    }

    #[test]
    fn example_3() {
        assert_eq!(
            2,
            Solution::min_refuel_stops(
                100,
                10,
                vec![vec![10, 60], vec![20, 30], vec![30, 30], vec![60, 40]]
            )
        );
    }
}
