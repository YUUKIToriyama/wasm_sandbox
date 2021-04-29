pub fn fibonacci(n: i32) -> i32 {
	fn calc(left: i32, right: i32, m: i32) -> i32 {
		if m == 0 {
			return left;
		} else {
			return calc(left + right, left, m - 1);
		}
	}
	return calc(1, 0, n);
}
pub fn fibonacci_match(n: i32) -> i32 {
	match n {
		0 => 0,
		1 => 1,
		_ => fibonacci(n - 2) + fibonacci(n - 1)
	}
}