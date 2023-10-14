pub struct Integer {
	value: i32,
}

impl From<i32> for Integer {
	fn from(value: i32) -> Self {
		Integer { value }
	}
}

impl Integer {
	pub fn value(&self) -> &i32 {
		&self.value
	}
}
