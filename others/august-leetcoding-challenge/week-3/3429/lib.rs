pub struct Solution {}

impl Solution {
    #[allow(clippy::wrong_self_convention)]
    pub fn to_goat_latin(s: String) -> String {
        let mut v: Vec<String> = Vec::new();
        for (i, w) in s.split(' ').enumerate() {
            let mut s = w.to_string();
            match &w.to_lowercase()[0..1] {
                "a" | "e" | "i" | "o" | "u" => {}
                _ => {
                    let c = s.remove(0);
                    s.push(c);
                }
            }
            s += "ma";
            for _ in 0..=i {
                s.push('a');
            }
            v.push(s);
        }
        v.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            "Imaa peaksmaaa oatGmaaaa atinLmaaaaa",
            Solution::to_goat_latin("I speak Goat Latin".to_string())
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            "heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa",
            Solution::to_goat_latin("The quick brown fox jumped over the lazy dog".to_string())
        );
    }
}
