pub struct Solution {}

impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut v: Vec<Vec<i32>> = people;
        v.sort_by(|a: &Vec<i32>, b: &Vec<i32>| match b[0].cmp(&a[0]) {
            std::cmp::Ordering::Equal => a[1].cmp(&b[1]),
            ord => ord,
        });
        let mut answer: Vec<Vec<i32>> = Vec::new();
        for p in v.iter() {
            answer.insert(p[1] as usize, p.clone());
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
            vec![
                vec![5, 0],
                vec![7, 0],
                vec![5, 2],
                vec![6, 1],
                vec![4, 4],
                vec![7, 1]
            ],
            Solution::reconstruct_queue(vec![
                vec![7, 0],
                vec![4, 4],
                vec![7, 1],
                vec![5, 0],
                vec![6, 1],
                vec![5, 2]
            ])
        );
    }
}
