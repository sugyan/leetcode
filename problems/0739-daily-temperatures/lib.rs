pub struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut answer = vec![0; temperatures.len()];
        let mut hottest = 0;
        for (i, &t) in temperatures.iter().enumerate().rev() {
            if t >= hottest {
                hottest = t;
                continue;
            }
            let mut days = 1;
            while temperatures[i + days] <= t {
                days += answer[i + days] as usize;
            }
            answer[i] = days as i32;
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
            vec![1, 1, 4, 2, 1, 1, 0, 0],
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![1, 1, 1, 0],
            Solution::daily_temperatures(vec![30, 40, 50, 60])
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            vec![1, 1, 0],
            Solution::daily_temperatures(vec![30, 60, 90])
        );
    }
}
