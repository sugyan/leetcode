pub struct Solution {}

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut v: Vec<Vec<char>> = nums
            .iter()
            .map(|n| n.to_string().chars().collect())
            .collect();
        let cmp = |a: &Vec<char>, b: &Vec<char>| -> std::cmp::Ordering {
            for i in 0..std::cmp::min(a.len(), b.len()) {
                if a[i] != b[i] {
                    return b[i].cmp(&a[i]);
                }
            }
            if a.len() != b.len() {
                let ab: Vec<&char> = a.iter().chain(b.iter()).collect();
                let ba: Vec<&char> = b.iter().chain(a.iter()).collect();
                return ba.cmp(&ab);
            }
            std::cmp::Ordering::Equal
        };
        v.sort_by(cmp);
        let mut answer: Vec<char> = v.into_iter().flatten().collect();
        while answer.len() > 1 && answer[0] == '0' {
            answer.remove(0);
        }
        answer.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!("210", Solution::largest_number(vec![10, 2]));
    }

    #[test]
    fn example_2() {
        assert_eq!("9534330", Solution::largest_number(vec![3, 30, 34, 5, 9]));
    }
}
