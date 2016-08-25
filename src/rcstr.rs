use std::rc::Rc;

pub struct RcSliceableString {
	string: Rc<String>,
	start: usize,
	len: usize,
}

impl RcSliceableString {
	pub fn new(s: String) -> RcSliceableString {
		let len = s.len();
		RcSliceableString {
			string: Rc::new(s),
			start: 0,
			len: len
		}
	}

	pub fn slice(&self, start: usize, len: usize) -> RcSliceableString {
		//bounds check
		let new_start = start;
		let new_end = start + len;

		assert!(new_start >= self.start);
		assert!(new_end <= self.start + self.len);

		RcSliceableString {
			string: self.string.clone(),
			start: start,
			len: len
		}
	}

	pub fn as_str(&self) -> &str {
		&self.string[self.start..self.start+self.len]
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_rc_sliceable_string() {
		let s_base = RcSliceableString::new("Hello world".to_owned());
		let s_sliced = s_base.slice(0, 5);
		assert_eq!(s_sliced.as_str(), "Hello");
	}
}