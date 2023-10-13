use library::integer::Integer;
use std::ops::{Add, Deref, DerefMut};
struct MyInteger(Integer);

impl Deref for MyInteger {
    type Target = Integer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<i32> for MyInteger {
    fn from(value: i32) -> Self {
        MyInteger(Integer::from(value))
    }
}

impl From<Integer> for MyInteger {
    fn from(value: Integer) -> Self {
        MyInteger(value)
    }
}

impl DerefMut for MyInteger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Add for MyInteger {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        MyInteger(Integer::from(self.value() + rhs.value()))
    }
}

fn main() {
    let a = MyInteger::from(32);
    let b = MyInteger::from(10);

    //Code completion was displaied "assert_i32"
    a.assert_i32(&42)
}

#[cfg(test)]
mod tests {
    use super::*;
    use library::integer_test_helper;

    #[test]
    fn test_add() {
        let a = MyInteger::from(32);
        let b = MyInteger::from(10);
        let c = a + b;

        c.assert_i32(&42)
    }
}
