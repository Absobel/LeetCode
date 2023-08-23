struct Solution;

fn main() {
    assert_eq!(Solution::convert("ABCDEF".to_string(), 5), "ABCDFE".to_string());
}

////////////////////////////////////////////////////////////////////////

fn return_every_nth_char(s: &str, n: i32) -> String {
    let mut res = String::new();
    for (i, c) in s.chars().enumerate() {
        if i % n as usize == 0 {
            res.push(c);
        }
    }
    res
}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if s.len() == 1 || num_rows == 1 || num_rows as usize >= s.len() {
            return s;
        }
        let mut res = String::new();
        res.push_str(&return_every_nth_char(&s, 2*num_rows-2));
        for i in 1..=num_rows-2 {
            let tmp1 = return_every_nth_char(&s[i as usize..], 2*num_rows-2);
            let mut tmp1_chars = tmp1.chars();
            if 2*num_rows as usize-2-(i as usize) < s.len() {
                let tmp2 = return_every_nth_char(&s[2*num_rows as usize-2-i as usize..], 2*num_rows-2);
                let mut tmp2_chars = tmp2.chars();
                for _ in 0..tmp1.len().min(tmp2.len()) {
                    res.push(tmp1_chars.next().unwrap());
                    res.push(tmp2_chars.next().unwrap());
                }
                res.extend(tmp2_chars);
            }
            res.extend(tmp1_chars);
        }
        res.push_str(&return_every_nth_char(&s[num_rows as usize-1..], 2*num_rows-2));
        res
    }
}

////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::convert("PAYPALISHIRING".to_string(), 3), "PAHNAPLSIIGYIR".to_string());
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::convert("PAYPALISHIRING".to_string(), 4), "PINALSIGYAHRPI".to_string());
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::convert("A".to_string(), 1), "A".to_string());
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::convert("A".to_string(), 3), "A".to_string());
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::convert("AB".to_string(), 1), "AB".to_string());
    }

    #[test]
    fn test_6() {
        assert_eq!(Solution::convert("ABCDEF".to_string(), 5), "ABCDFE".to_string());
    }
}
