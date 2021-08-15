pub struct Solution;

impl Solution {
    pub fn remove_boxes(boxes: Vec<i32>) -> i32 {
        let len = boxes.len();
        let mut memo = vec![vec![vec![None; len]; len]; len];
        Self::dp(&boxes, &mut memo, 0, len - 1, 0)
    }
    fn dp(
        boxes: &[i32],
        memo: &mut Vec<Vec<Vec<Option<i32>>>>,
        i: usize,
        j: usize,
        k: usize,
    ) -> i32 {
        if let Some(val) = memo[i][j][k] {
            return val;
        }
        let mut mi = i;
        let mut mk = k as i32;
        while mi < j && boxes[mi + 1] == boxes[mi] {
            mi += 1;
            mk += 1;
        }
        let mut max = (mk + 1) * (mk + 1)
            + if mi < j {
                Self::dp(boxes, memo, mi + 1, j, 0)
            } else {
                0
            };
        for l in mi + 1..=j {
            if boxes[l] == boxes[i] {
                max = max.max(
                    Self::dp(boxes, memo, mi + 1, l - 1, 0)
                        + Self::dp(boxes, memo, l, j, mk as usize + 1),
                )
            }
        }
        memo[i][j][k] = Some(max);
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(23, Solution::remove_boxes(vec![1, 3, 2, 2, 2, 3, 4, 3, 1]));
    }

    #[test]
    fn example_2() {
        assert_eq!(9, Solution::remove_boxes(vec![1, 1, 1]));
    }

    #[test]
    fn example_3() {
        assert_eq!(1, Solution::remove_boxes(vec![1]));
    }
}
