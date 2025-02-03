#[cfg(test)]
pub mod tests {
	use wordle::*;
	#[test]
	fn load_test() {
		let _ = load("words.txt".to_string());
	}

	#[test]
	fn cmp_all_grey() {
		let a = "AAAAA".chars().collect::<Vec<_>>();
		let b = "BBBBB".chars().collect::<Vec<_>>();

		let res = cmp_words(&a, &b);
		assert!(res.iter().all(|f| { *f == Color::Grey }));
	}

	#[test]
	fn cmp_all_green() {
		let a = "AAAAA".chars().collect::<Vec<_>>();
		let b = "AAAAA".chars().collect::<Vec<_>>();

		let res = cmp_words(&a, &b);
		assert!(res.iter().all(|f| { *f == Color::Green }));
	}

	#[test]
	fn cmp_green_grey() {
		let a = "ABCDE".chars().collect::<Vec<_>>();
		let b = "AXCXE".chars().collect::<Vec<_>>();
		let res = cmp_words(&a, &b);
		let real = vec![
			Color::Green,
			Color::Grey,
			Color::Green,
			Color::Grey,
			Color::Green,
		];
		assert!(res.iter().zip(real).all(|(a, b)| { *a == b }));
	}

	#[test]
	fn cmp_yellow() {
		let a = "ABCDE".chars().collect::<Vec<_>>();
		let b = "XAAAA".chars().collect::<Vec<_>>();
		let res = cmp_words(&a, &b);
		let real = vec![
			Color::Grey,
			Color::Yellow,
			Color::Grey,
			Color::Grey,
			Color::Grey,
		];
		assert!(res.iter().zip(real).all(|(a, b)| { *a == b }));
	}

	#[test]
	fn cmp_yellow_2() {
		let a = "AABCD".chars().collect::<Vec<_>>();
		let b = "XXAAA".chars().collect::<Vec<_>>();
		let res = cmp_words(&a, &b);
		let real = vec![
			Color::Grey,
			Color::Grey,
			Color::Yellow,
			Color::Yellow,
			Color::Grey,
		];
		assert!(res.iter().zip(real).all(|(a, b)| { *a == b }));
	}

	#[test]
	fn numres() {
		for i in 0..(LEN) {
			assert!(res_to_num(num_to_res(i)) == i);
		}
	}
}
