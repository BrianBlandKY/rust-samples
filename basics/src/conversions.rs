use std::fmt;
use std::convert::From;

#[derive(Debug)]
pub struct Number {
    value: i32,
}

impl fmt::Display for Number {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.value)
    }
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

#[cfg(test)]
mod tests {
    use crate::conversions::Number;
    #[test]
    fn test_from_to() {

        let v = Number::from(42);
        println!("v: {}", v);

        let x : Number = v.into(); 
        println!("x: {}", x);
    }
}
