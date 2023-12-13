use core::num;
use std::{
    error::Error,
    fmt::{write, Display},
    str::FromStr,
    string::ParseError,
};

use crate::util;

pub trait LargeNumber {
    fn is_valid(&self) -> bool;

    fn parse<F: FromStr>(&self) -> Result<F, F::Err>;

    fn add(&self, second: Self) -> Result<Self, LargeNumberError>
    where
        Self: Sized;

    fn sub(&self, second: Self) -> Self;

    fn divide(&self, second: Self) -> Self;

    fn multiply(&self, second: Self) -> Self;

    fn push(&mut self, ch: char);
}

#[derive(Debug, PartialEq, Eq)]
pub struct LargeInteger {
    pub val: String,
}

impl LargeInteger {
    pub fn new() -> Self {
        Self { val: String::new() }
    }
}

impl From<String> for LargeInteger {
    fn from(value: String) -> Self {
        Self { val: value }
    }
}

impl From<&str> for LargeInteger {
    fn from(value: &str) -> Self {
        Self { val: value.into() }
    }
}

impl LargeNumber for LargeInteger {
    fn is_valid(&self) -> bool {
        self.val.chars().all(|c| c.is_digit(10))
    }

    fn parse<F: FromStr>(&self) -> Result<F, F::Err> {
        self.val.parse()
    }

    fn add(&self, second: Self) -> Result<Self, LargeNumberError> {
        // use two if to allow for more precise error description
        if self.is_valid() {
            if second.is_valid() {
                // first large number's value as a character iterator
                let num1 = self.val.chars().rev();
                // second large number's value as a character iterator
                let num2 = second.val.chars().rev();
                // if a result returns more than 9,
                // the second digit of that result goes to the buffer
                let mut buf: Option<u8> = None;
                // end result with type LargeInteger
                let mut res = LargeInteger::new();
                num1.zip(num2).for_each(|(ch1, ch2)| {
                    // check if both numbers are 0 so we can just push 0 without doing any calculations
                    if ch1 != '0' || ch2 != '0' && buf == None {
                        // unwrap is safe as we check if all character are digits using the is_valid checks

                        // single character from num1 as a digit
                        let ch1_num = ch1.to_digit(10).unwrap();
                        // single character from num2 as a digit
                        let ch2_num = ch2.to_digit(10).unwrap();

                        // raw result of the addition of ch1_num and ch2_num
                        let temp_res = ch1_num + ch2_num;

                        // check if the temp result needs to push overflow to the buffer
                        if temp_res < 10 {
                            res.push(util::uint_to_char(temp_res));
                        } else {
                            let vec_char = util::uint_to_short_vec(temp_res);
                            res.push(*vec_char.last().unwrap())
                        }
                    } else {
                        res.push('0')
                    }
                });
                let res: String = res.val.chars().rev().collect();
                return Ok(LargeInteger::from(res));
            }
            return Err(LargeNumberError(
                "Failed to add two large numbers as the first one is not valid",
            ));
        }
        return Err(LargeNumberError(
            "Failed to add two large numbers as the first one is not valid",
        ));
    }

    fn sub(&self, second: Self) -> Self {
        todo!()
    }

    fn divide(&self, second: Self) -> Self {
        todo!()
    }

    fn multiply(&self, second: Self) -> Self {
        todo!()
    }

    fn push(&mut self, ch: char) {
        self.val.push(ch)
    }
}

#[derive(Debug)]
pub struct LargeNumberError<'a>(&'a str);

impl Display for LargeNumberError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

impl Error for LargeNumberError<'_> {}
