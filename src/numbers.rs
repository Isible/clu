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
        Self {
            val: String::new(),
        }
    }
}

impl From<String> for LargeInteger {
    fn from(value: String) -> Self {
        Self {
            val: value
        }
    }
}

impl From<&str> for LargeInteger {
    fn from(value: &str) -> Self {
        Self {
            val: value.into()
        }
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
                let num1 = self.val.chars().rev();
                let num2 = second.val.chars().rev();
                let mut res = LargeInteger::new();
                num1.zip(num2).for_each(|(ch1, ch2)| {
                    if ch1 != '0' || ch2 != '0' {
                        // unwrap is safe as we check if all character are digits using the is_valid checks
                        let ch1_num = ch1.to_digit(10).unwrap();
                        let ch2_num = ch2.to_digit(10).unwrap();
                        dbg!("{}, {}", ch1_num, ch2_num);
                        let temp_res = ch1_num + ch2_num;
                        dbg!("{}", temp_res);
                        if temp_res < 10 {
                            res.push(util::uint_to_char(temp_res));
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

pub trait Number {
    fn test(&self) {

    }
}

/// implements [`Number`](crate::Number) for the provided types
macro_rules! impl_number {
    ($($ty:ty),*) => {
        $(impl Number for $ty {})*
    };
}

// rust's numbers
impl_number!(u8, u16, u32, u64, u128);
impl_number!(i8, i16, i32, i64, i128);
impl_number!(f32, f64);

// clu's numbers
impl_number!(LargeInteger);
