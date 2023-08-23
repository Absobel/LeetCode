struct Solution;

fn main() {
    println!("{}", Solution::reverse(-2147483648));
}

////////////////////////////////////////////////////////////////////////

struct Deconstruction {
    sign: i32,
    digits: Vec<i32>,
}

impl From<i32> for Deconstruction {
    fn from(value: i32) -> Self {
        if value == i32::MIN {
            return Self {
                sign: -1,
                digits: vec![2, 1, 4, 7, 4, 8, 3, 6, 4, 8],
            };
        }
        let sign = value.signum();
        let digits = value
            .abs()
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect::<Vec<_>>();

        Self { sign, digits }
    }
}

impl Deconstruction {
    fn reverse(mut self) -> Self {
        self.digits.reverse();
        self
    }
}

impl From<Deconstruction> for i32 {
    fn from(value: Deconstruction) -> Self {
        let mut res = 0;
        for digit in value.digits {
            let overflow;
            (res, overflow) = 10_i32.overflowing_mul(res);
            if overflow {
                return 0;
            } else {
                let overflow;
                (res, overflow) = res.overflowing_add(digit);
                if overflow {
                    return 0;
                }
            }
        }
        res * value.sign
    }
}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        Deconstruction::from(x).reverse().into()
    }
}

////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn t1() {
        assert_eq!(Solution::reverse(123), 321);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::reverse(-123), -321);
    }

    #[test]
    fn t3() {
        assert_eq!(Solution::reverse(120), 21);
    }

    #[test]
    fn t4() {
        assert_eq!(Solution::reverse(1534236469), 0);
    }
}
