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

    //Code completion was displaied "assert_i32"
    //But can't compile.

    // error[E0599]: no method named `assert_i32` found for struct `MyInteger` in the current scope
    //   --> playground\src\main.rs:44:7
    //   |
    //   3  | struct MyInteger(Integer);
    // | ---------------- method `assert_i32` not found for this struct
    // ...
    //   44 |     a.assert_i32(&42)
    //     |       ^^^^^^^^^^ method not found in `MyInteger`
    a.assert_i32(&42)
}

#[cfg(test)]
mod tests {
    use super::*;

    //Can compile and run test successfully.
    #[test]
    fn test_add() {
        let a = MyInteger::from(32);
        let b = MyInteger::from(10);
        let c = a + b;

        c.assert_i32(&42)
    }
}
