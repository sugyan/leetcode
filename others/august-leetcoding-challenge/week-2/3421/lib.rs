pub struct Solution {}

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut answer = Vec::with_capacity(row_index as usize + 1);
        answer.push(1);
        for i in 0..row_index as usize {
            answer.push(1);
            for j in (1..=i).rev() {
                answer[j] += answer[j - 1];
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
        assert_eq!(vec![1, 3, 3, 1], Solution::get_row(3));
    }
}
