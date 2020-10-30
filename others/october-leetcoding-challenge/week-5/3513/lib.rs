pub struct Solution {}

impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let mut l: Vec<usize> = vec![0; nums.len()];
        let mut c: Vec<usize> = vec![1; nums.len()];
        for i in 0..nums.len() {
            for j in 0..i {
                if nums[j] < nums[i] {
                    if l[j] >= l[i] {
                        l[i] = l[j] + 1;
                        c[i] = c[j];
                    } else if l[j] + 1 == l[i] {
                        c[i] += c[j];
                    }
                }
            }
        }
        let mut answer = 0;
        if let Some(&longest) = l.iter().max() {
            for i in 0..nums.len() {
                if l[i] == longest {
                    answer += c[i];
                }
            }
        }
        answer as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(2, Solution::find_number_of_lis(vec![1, 3, 5, 4, 7]));
    }

    #[test]
    fn example_2() {
        assert_eq!(5, Solution::find_number_of_lis(vec![2, 2, 2, 2, 2]));
    }
}
