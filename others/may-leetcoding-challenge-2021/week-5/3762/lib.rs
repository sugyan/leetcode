pub struct Solution;

impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut products = products;
        products.sort();
        let search_word = search_word.as_bytes();
        let mut answer = vec![Vec::new(); search_word.len()];
        for product in &products {
            let p = product.as_bytes();
            let mut i = 0;
            while i < p.len().min(search_word.len()) && p[i] == search_word[i] {
                if answer[i].len() < 3 {
                    answer[i].push(product.clone());
                }
                i += 1;
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
        assert_eq!(
            vec![
                vec!["mobile", "moneypot", "monitor"],
                vec!["mobile", "moneypot", "monitor"],
                vec!["mouse", "mousepad"],
                vec!["mouse", "mousepad"],
                vec!["mouse", "mousepad"]
            ],
            Solution::suggested_products(
                vec!["mobile", "mouse", "moneypot", "monitor", "mousepad"]
                    .into_iter()
                    .map(String::from)
                    .collect(),
                String::from("mouse")
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![
                vec!["havana"],
                vec!["havana"],
                vec!["havana"],
                vec!["havana"],
                vec!["havana"],
                vec!["havana"]
            ],
            Solution::suggested_products(
                vec!["havana"].into_iter().map(String::from).collect(),
                String::from("havana")
            )
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            vec![
                vec!["baggage", "bags", "banner"],
                vec!["baggage", "bags", "banner"],
                vec!["baggage", "bags"],
                vec!["bags"]
            ],
            Solution::suggested_products(
                vec!["bags", "baggage", "banner", "box", "cloths"]
                    .into_iter()
                    .map(String::from)
                    .collect(),
                String::from("bags")
            )
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            vec![
                Vec::<String>::new(),
                Vec::<String>::new(),
                Vec::<String>::new(),
                Vec::<String>::new(),
                Vec::<String>::new(),
                Vec::<String>::new(),
                Vec::<String>::new()
            ],
            Solution::suggested_products(
                vec!["havana"].into_iter().map(String::from).collect(),
                String::from("tatiana")
            )
        );
    }
}
