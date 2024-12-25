use std::ops::{Add, Sub};

struct BigInt {
    buf: Vec<u8>,
}

impl BigInt {
    fn new(number: &str) -> BigInt {
        let mut res: BigInt = BigInt{buf: Vec::new()};

        for ch in number.chars() {
            let digit = ch as u8 - b'0';
            res.buf.push(digit);
        }

        res
    }
}

impl std::fmt::Display for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for digit in &self.buf {
            match write!(f, "{digit}") {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }
}

impl Add<BigInt> for BigInt {
    type Output = BigInt;

    fn add(mut self, mut rhs: BigInt) -> BigInt {
        let mut temp: Vec<u8> = Vec::new();
        let mut carry: u8 = 0;

        if self.buf.len() != rhs.buf.len() {
            if self.buf.len() > rhs.buf.len() {
                let diff = self.buf.len() - rhs.buf.len();
                for _ in 0..diff {
                    rhs.buf.insert(0, 0);
                }
            } else {
                let diff =  rhs.buf.len() - self.buf.len();
                for _ in 0..diff {
                    self.buf.insert(0, 0);
                }
            }
        }

        for (i, self_digit) in self.buf.iter().rev().enumerate() {
            let rhs_digit = rhs.buf.iter().rev().nth(i).unwrap();
            let new_digit = self_digit + rhs_digit + carry;
            carry = (new_digit / 10) as u8 % 10;
            let unit = new_digit % 10;
            temp.push(unit);
        }

        if carry != 0 {
            let mut last_digit = temp.pop().unwrap();
            last_digit += carry;
            temp.push(last_digit);
        }

        temp.reverse();
        BigInt{ buf: temp }
    }
}

impl Sub<BigInt> for BigInt {
    type Output = BigInt;

    fn sub(mut self, mut rhs: BigInt) -> BigInt {
        let mut temp: Vec<u8> = Vec::new();

        if self.buf.len() != rhs.buf.len() {
            if self.buf.len() > rhs.buf.len() {
                let diff = self.buf.len() - rhs.buf.len();
                for _ in 0..diff {
                    rhs.buf.insert(0, 0);
                }
            } else {
                let diff =  rhs.buf.len() - self.buf.len();
                for _ in 0..diff {
                    self.buf.insert(0, 0);
                }
            }
        }

        let mut decrease = false;
        for (i, self_digit) in self.buf.iter().rev().enumerate() {
            let rhs_digit = rhs.buf.iter().rev().nth(i).unwrap();

            let digit = if decrease {
                if self_digit == &0 {
                    9
                } else {
                    self_digit - 1
                }
            } else {
                *self_digit
            };

            let new_digit = if digit > *rhs_digit {
                digit - rhs_digit
            } else if digit == *rhs_digit {
                0
            } else {
                println!("digit: {digit}, rhs_digit: {rhs_digit}");
                decrease = true;
                (digit + 10) - rhs_digit
            };
            temp.push(new_digit);
        }

        temp.reverse();
        let mut res = Vec::new();
        let mut seen_non_zero = false;
        let mut zeros = Vec::new();
        for num in &temp {
            if num == &0 && !seen_non_zero {
                zeros.push(num.clone());
                continue;
            }

            if num == &0 && seen_non_zero {
                res.push(num.clone());
            } else {
                seen_non_zero = true;
                res.push(num.clone());
            }
        }

        if !seen_non_zero && zeros.len() == 1 {
            res = vec![0];
        }

        BigInt { buf: res }
    }
}

fn main() {
    // rust cannot do this since it'll overflow
    // let a: i128 = 340282366920938463463374607431768211455 + 340282366920938463463374607431768211455;

    let x = BigInt::new("2");
    let y = BigInt::new("2");
    let z = x - y;
    println!("{}", z);
}
