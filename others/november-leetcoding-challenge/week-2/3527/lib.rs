pub struct Solution {}

impl Solution {
    pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
        let v12 = vec![p2[0] - p1[0], p2[1] - p1[1]];
        let v13 = vec![p3[0] - p1[0], p3[1] - p1[1]];
        let v14 = vec![p4[0] - p1[0], p4[1] - p1[1]];
        if (v12[0] == 0 && v12[1] == 0)
            || (v13[0] == 0 && v13[1] == 0)
            || (v14[0] == 0 && v14[1] == 0)
        {
            return false;
        }
        if v12[0] * v13[0] + v12[1] * v13[1] == 0
            && v12[0] * v12[0] + v12[1] * v12[1] == v13[0] * v13[0] + v13[1] * v13[1]
            && (p1[0] + v12[0] + v13[0] == p4[0] && p1[1] + v12[1] + v13[1] == p4[1])
        {
            return true;
        }
        if v13[0] * v14[0] + v13[1] * v14[1] == 0
            && v13[0] * v13[0] + v13[1] * v13[1] == v14[0] * v14[0] + v14[1] * v14[1]
            && (p1[0] + v13[0] + v14[0] == p2[0] && p1[1] + v13[1] + v14[1] == p2[1])
        {
            return true;
        }
        if v14[0] * v12[0] + v14[1] * v12[1] == 0
            && v14[0] * v14[0] + v14[1] * v14[1] == v12[0] * v12[0] + v12[1] * v12[1]
            && (p1[0] + v14[0] + v12[0] == p3[0] && p1[1] + v14[1] + v12[1] == p3[1])
        {
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            true,
            Solution::valid_square(vec![0, 0], vec![1, 1], vec![1, 0], vec![0, 1])
        );
    }
}
