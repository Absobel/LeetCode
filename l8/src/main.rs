struct Solution;

fn main() {
    println!("{}", Solution::my_atoi("42".to_string()));
}

////////////////////////////////////////////////////////////////////////

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        
    }
}

////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn t1() {
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::my_atoi("   -42".to_string()), -42);
    }

    #[test]
    fn t3() {
        assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
    }
}
