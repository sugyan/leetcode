pub struct Solution {}

impl Solution {
    pub fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
        let mut i = 0;
        while i < arr.len() {
            if let Some(piece) = pieces.iter().find(|&piece| piece[0] == arr[i]) {
                if piece
                    .iter()
                    .enumerate()
                    .skip(1)
                    .any(|(j, &p)| p != arr[i + j])
                {
                    return false;
                }
                i += piece.len();
            } else {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(true, Solution::can_form_array(vec![85], vec![vec![85]]));
    }

    #[test]
    fn example_2() {
        assert_eq!(
            true,
            Solution::can_form_array(vec![15, 88], vec![vec![88], vec![15]])
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            false,
            Solution::can_form_array(vec![49, 18, 16], vec![vec![16, 18, 49]])
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            true,
            Solution::can_form_array(vec![91, 4, 64, 78], vec![vec![78], vec![4, 64], vec![91]])
        );
    }

    #[test]
    fn example_5() {
        assert_eq!(
            false,
            Solution::can_form_array(vec![1, 3, 5, 7], vec![vec![2, 4, 6, 8]])
        );
    }
}
