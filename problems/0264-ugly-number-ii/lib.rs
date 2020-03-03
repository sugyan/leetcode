pub struct Solution {}

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut v: Vec<i32> = Vec::with_capacity(1690);
        v.push(1);
        let mut p = (0, 0, 0);
        while v.len() < n as usize {
            let nums = [v[p.0] * 2, v[p.1] * 3, v[p.2] * 5];
            let min = nums.iter().min().unwrap();
            v.push(*min);
            if *min == nums[0] {
                p.0 += 1;
            }
            if *min == nums[1] {
                p.1 += 1;
            }
            if *min == nums[2] {
                p.2 += 1;
            }
        }
        v[v.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(12, Solution::nth_ugly_number(10));
    }
}
