use dashmap::DashMap;
use itertools::Itertools;
use lazy_static::lazy_static;
use std::i32;
use std::sync::Mutex;
use std::{collections::HashMap, fs};

pub const SIZE: usize = 5;
pub const LEN: i32 = 3_i32.pow(SIZE as u32);
pub const MAX: usize = 14855;

#[derive(Debug, PartialEq, Clone, Eq, Copy, Hash)]
pub enum Color {
	Grey,
	Yellow,
	Green,
}

// pub fn gen_color_perms() -> Vec<[Color; SIZE]> {
// 	// pub fn gen_color_perms() -> Vec<Vec<Color>> {
// 	let colors = [Color::Green, Color::Yellow, Color::Grey];
// 	let colors = colors
// 		.iter()
// 		.flat_map(|&x| std::iter::repeat(x).take(SIZE))
// 		.collect::<Vec<_>>();
// 	// colors.into_iter().permutations(SIZE).unique().collect()
// 	colors
// 		.into_iter()
// 		.permutations(SIZE)
// 		.unique()
// 		// .par_bridge()
// 		.map(|perm| {
// 			let mut array = [Color::Green; SIZE];
// 			for (i, &color) in perm.iter().enumerate() {
// 				array[i] = color;
// 			}
// 			array
// 		})
// 		.collect()
// }

pub fn gen_color_perms() -> HashMap<[Color; SIZE], Mutex<i32>> {
	// pub fn gen_color_perms() -> HashMap<[Color; SIZE], AtomicI32> {
	let colors = [Color::Green, Color::Yellow, Color::Grey]; // Initial fixed-size array

	let colors = colors
		.iter()
		.flat_map(|&x| std::iter::repeat(x).take(SIZE))
		.collect::<Vec<_>>();
	// for i in 0..SIZE {
	// 	repeated_colors[i] = colors[i]; // Repeating each color
	// }

	colors
		.into_iter()
		.permutations(SIZE)
		// .par_bridge()
		.map(|perm| {
			let mut array = [Color::Green; SIZE];
			for (i, &color) in perm.iter().enumerate() {
				array[i] = color;
			}
			(array, Mutex::new(0))
			// (array, AtomicI32::new(0))
		})
		.collect()
}

// pub fn to_word(a: String) -> [char; SIZE]{
// 	return a.chars().collect::<Vec<_>>();;
// }

// pub fn load(path: String) -> Vec<Vec<char>> {
// 	fs::read_to_string(path)
// 		.unwrap()
// 		.lines()
// 		// .par_bridge()
// 		.map(|line| line.chars().collect())
// 		.collect()
// }

pub fn load(path: String) -> Vec<[char; SIZE]> {
	fs::read_to_string(path)
		.unwrap()
		.lines()
		.map(|line| {
			let mut array = [' '; SIZE];
			for (i, c) in line.chars().take(5).enumerate() {
				array[i] = c;
			}
			array
		})
		.collect()
}

pub fn load_str(path: String) -> Vec<String> {
	fs::read_to_string(path)
		.unwrap()
		.lines()
		.map(String::from)
		.collect()
}

pub fn convert(a: String) -> Vec<char> {
	a.chars().collect()
}
pub fn convert1(a: &str) -> [char; 5] {
	a.to_string()
		.chars()
		.collect::<Vec<_>>()
		.try_into()
		.unwrap()
}
pub fn convert2(a: String) -> [char; 5] {
	a.to_string()
		.chars()
		.collect::<Vec<_>>()
		.try_into()
		.unwrap()
}

pub fn char_to_str(a: &[char]) -> String {
	a.iter().collect()
}

pub fn cmp_words_str(real: &String, input: &String) -> Vec<Color> {
	real.chars()
		.zip(input.chars())
		.map(|(a, b)| if a == b { Color::Green } else { Color::Grey })
		.collect()
	// todo!();
}

pub fn color_to_str(a: [Color; SIZE]) -> String {
	a.iter()
		.map(|f| match f {
			Color::Grey => "⬜",
			Color::Yellow => "🟨",
			Color::Green => "🟩",
		})
		// .rev()
		.collect()
}

pub fn print_color(a: [Color; SIZE]) {
	println!("{}", color_to_str(a));
}

lazy_static! {
	static ref map: DashMap<([char; SIZE], [char; SIZE]), [Color; SIZE]> =
		DashMap::<([char; SIZE], [char; SIZE]), [Color; SIZE]>::new();
}

pub fn cmp_words_hash(real: &[char; SIZE], input: &[char; SIZE]) -> [Color; SIZE] {
	if let Some(x) = map.get(&(*real, *input)) {
		return *x;
	}
	let x = cmp_words(real, input);
	{
		map.insert((*real, *input), x);
	}
	x
}

pub fn cmp_words(real: &[char], input: &[char]) -> [Color; SIZE] {
	let mut result = [Color::Grey; SIZE];
	let mut count_a = std::collections::HashMap::new();

	for (i, &letter_a) in real.iter().enumerate() {
		let letter_b = input[i];

		if letter_a == letter_b {
			result[i] = Color::Green;
		} else {
			*count_a.entry(letter_a).or_insert(0) += 1;
		}
	}

	for (i, &letter_b) in input.iter().enumerate() {
		if result[i] == Color::Green {
			continue;
		}

		if count_a.get(&letter_b).unwrap_or(&0) > &0 {
			result[i] = Color::Yellow;
			*count_a.entry(letter_b).or_insert(0) -= 1;
		}
	}

	result
}

pub fn res_to_num(a: [Color; SIZE]) -> i32 {
	let mut total = 0;
	for i in 0..SIZE {
		total += match a.get(i).unwrap() {
			Color::Grey => 0,
			Color::Yellow => 1,
			Color::Green => 2,
		} * 3_i32.pow(i as u32);
	}
	total
}

pub fn num_to_res(num: i32) -> [Color; SIZE] {
	let mut result = [Color::Grey; SIZE];
	let mut temp = num;

	for i in 0..SIZE {
		let value = temp % 3;
		result[i] = match value {
			0 => Color::Grey,
			1 => Color::Yellow,
			2 => Color::Green,
			_ => unreachable!(),
		};
		temp /= 3;
	}

	result
}
