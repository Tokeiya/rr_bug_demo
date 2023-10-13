use crate::integer::Integer;

impl Integer {
    pub fn assert_i32(&self, expected: &i32) {
        assert_eq!(self.value(), expected);
    }
}