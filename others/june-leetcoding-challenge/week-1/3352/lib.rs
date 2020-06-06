pub struct Solution {}

impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut v: Vec<Vec<i32>> = people;
        v.sort_by(|a: &Vec<i32>, b: &Vec<i32>| match a[0].cmp(&b[0]) {
            std::cmp::Ordering::Equal => b[1].cmp(&a[1]),
            ord => ord,
        });
        let mut index: Vec<usize> = (0..v.len()).collect();
        let mut answer: Vec<Vec<i32>> = vec![Vec::new(); v.len()];
        for p in v.iter() {
            answer[index[p[1] as usize]] = p.clone();
            index.remove(p[1] as usize);
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
