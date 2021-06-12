pub struct Solution;

impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![0; stations.len() + 1];
        dp[0] = start_fuel;
        for (i, station) in stations.iter().enumerate() {
            for j in (0..=i).rev() {
                if dp[j] >= station[0] {
                    dp[j + 1] = dp[j + 1].max(dp[j] + station[1]);
                }
            }
        }
        for (i, &d) in dp.iter().enumerate() {
            if d >= target {
                return i as i32;
            }
        }
        -1
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
