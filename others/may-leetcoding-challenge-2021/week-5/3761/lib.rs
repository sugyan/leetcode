pub struct Solution;

impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        let (min, max) = nums
            .iter()
            .fold((std::i32::MAX, std::i32::MIN), |(min, max), &x| {
                (min.min(x), max.max(x))
            });
        let gap = ((max - min) as f64 / (nums.len() - 1) as f64).ceil() as i32;
        let mut mins = vec![None; nums.len() - 1];
        let mut maxs = vec![None; nums.len() - 1];
        for &num in &nums {
            if num == min || num == max {
                continue;
            }
            let i = ((num - min) / gap) as usize;
            mins[i] = Some(mins[i].map_or(num, |m: i32| m.min(num)));
            maxs[i] = Some(maxs[i].map_or(num, |m: i32| m.max(num)));
        }
        let mut answer = 0;
        let mut prev = min;
        for i in 0..nums.len() - 1 {
            if let (Some(mini), Some(maxi)) = (mins[i], maxs[i]) {
                answer = answer.max(mini - prev);
                prev = maxi;
            }
        }
        answer.max(max - prev)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(3, Solution::maximum_gap(vec![3, 6, 9, 1]));
    }

    #[test]
    fn example_2() {
        assert_eq!(0, Solution::maximum_gap(vec![10]));
    }
}
