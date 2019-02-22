pub fn fizz_buzz() {
    let mut x;
    for i in 1..101 {
        let result = if i%15 == 0 {
            "FizzBuzz"
        } else if i % 5 == 0  {
            "Buzz"
        } else if i % 3 == 0 {
            "Fizz"
        } else {
            x = i.to_string();
            &*x
        };
        println!("{}", result);
    }
}

use std::borrow::Cow;

pub fn fizz_buzz_V2() {
    for i in 1..101 {
        let mut result: Cow<str> = if i%15 == 0 {
            "FizzBuzz".into()
        } else if i % 5 == 0  {
            "Buzz".into()
        } else if i % 3 == 0 {
            "Fizz".into()
        } else {
            i.to_string().into()
        };
        println!("{}", result);

    }
}

use std::fmt;

enum FizzBuzzItem {
    Fizz,
    Buzz,
    FizzBuzz,
    Number(i32),
}

impl fmt::Display for FizzBuzzItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match *self {
            FizzBuzzItem::Fizz => f.write_str("Fizz"),
            FizzBuzzItem::Buzz => f.write_str("Buzz"),
            FizzBuzzItem::FizzBuzz => f.write_str("FizzBuzz"),
            FizzBuzzItem::Number(num) => write!(f, "{}", num),
        }
    }
}

impl fmt::Debug for FizzBuzzItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

fn fizz_buzz_V3() {
    for i in 0..101 {
        let result = if i % 15 == 0 {
            FizzBuzzItem::FizzBuzz
        } else if i % 5 == 0 {
            FizzBuzzItem::Fizz
        } else if i % 3 == 0 {
            FizzBuzzItem::Buzz
        } else {
           FizzBuzzItem::Number(i)
        };
        println!("{}", result);
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_fizz_buzz() {
        super::fizz_buzz();
    }
}