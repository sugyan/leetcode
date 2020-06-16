pub struct Solution {}

impl Solution {
    pub fn valid_ip_address(ip: String) -> String {
        let s4: Vec<&str> = ip.split('.').collect();
        let s6: Vec<&str> = ip.split(':').collect();
        if s4.len() == 4
            && s4
                .iter()
                .all(|s| (s == &"0" || !s.starts_with('0')) && s.parse::<u8>().ok().is_some())
        {
            return "IPv4".to_string();
        }
        if s6.len() == 8
            && s6
                .iter()
                .all(|s| s.len() <= 4 && u16::from_str_radix(&s, 16).ok().is_some())
        {
            return "IPv6".to_string();
        }
        "Neither".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            "IPv4",
            Solution::valid_ip_address("172.16.254.1".to_string())
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            "IPv6",
            Solution::valid_ip_address("2001:0db8:85a3:0:0:8A2E:0370:7334".to_string())
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            "Neither",
            Solution::valid_ip_address("256.256.256.256".to_string())
        );
    }
}
