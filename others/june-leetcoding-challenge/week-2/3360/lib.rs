use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut graph: Vec<Vec<(usize, i32)>> = vec![vec![]; n as usize];
        for flight in flights.iter() {
            graph[flight[0] as usize].push((flight[1] as usize, flight[2]));
        }
        let mut vd: VecDeque<(usize, i32, usize)> = VecDeque::new();
        vd.push_back((src as usize, 0, 0));
        let mut answer = None;
        while let Some(front) = vd.pop_front() {
            if front.0 == dst as usize {
                answer = Some(if let Some(a) = answer {
                    std::cmp::min(a, front.1)
                } else {
                    front.1
                });
            }
            if let Some(a) = answer {
                if front.1 > a {
                    continue;
                }
            }
            if front.2 <= k as usize {
                for path in graph[front.0].iter() {
                    vd.push_back((path.0, front.1 + path.1, front.2 + 1));
                }
            }
        }
        if let Some(a) = answer {
            a
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            200,
            Solution::find_cheapest_price(
                3,
                vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
                0,
                2,
                1
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            500,
            Solution::find_cheapest_price(
                3,
                vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
                0,
                2,
                0
            )
        );
    }
}
