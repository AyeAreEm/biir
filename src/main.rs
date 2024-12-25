use std::ops::Add;

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

    fn add(self, mut rhs: BigInt) -> BigInt {
        let mut temp: Vec<u8> = Vec::new();
        let mut carry: u8 = 0;

        if self.buf.len() != rhs.buf.len() {
            if self.buf.len() > rhs.buf.len() {
                let diff = self.buf.len() - rhs.buf.len();
                for _ in 0..diff {
                    rhs.buf.insert(0, 0);
                }
            }
        }

        for (i, self_digit) in self.buf.iter().rev().enumerate() {
            let rhs_digit = match rhs.buf.iter().rev().nth(i) {
                Some(d) => d,
                None => {
                    temp.push(self_digit.clone() + carry);
                    continue;
                },
            };
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
        BigInt{ buf: temp.clone() }
    }
}

fn main() {
    // rust cannot do this since it'll overflow
    // let a: i128 = 340282366920938463463374607431768211455 + 340282366920938463463374607431768211455;

    let x = BigInt::new("340282366920938463463374607431768211456");
    let y = BigInt::new("340282366920938463463374607431768211455");
    let z = x + y;
    println!("{}", z);
}
