pub struct Solution {}

impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        let (r, c) = (image.len(), image[0].len());
        let mut image: Vec<Vec<i32>> = image;
        let mut s: Vec<(usize, usize)> = vec![(sr as usize, sc as usize)];
        while let Some(p) = s.pop() {
            if image[p.0][p.1] == new_color {
                continue;
            }
            if p.0 > 0 && image[p.0 - 1][p.1] == image[p.0][p.1] {
                s.push((p.0 - 1, p.1));
            }
            if p.1 > 0 && image[p.0][p.1 - 1] == image[p.0][p.1] {
                s.push((p.0, p.1 - 1));
            }
            if p.0 < r - 1 && image[p.0 + 1][p.1] == image[p.0][p.1] {
                s.push((p.0 + 1, p.1));
            }
            if p.1 < c - 1 && image[p.0][p.1 + 1] == image[p.0][p.1] {
                s.push((p.0, p.1 + 1));
            }
            image[p.0][p.1] = new_color;
        }
        image
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]],
            Solution::flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2)
        );
    }
}
