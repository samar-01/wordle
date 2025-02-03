use criterion::{criterion_group, criterion_main, Criterion};
use rand::random;
use rand::Rng;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use std::{
	collections::HashMap,
	sync::{atomic::AtomicI32, Mutex},
};
use wordle::*;

// use bench::play_game;

fn loadchar(c: &mut Criterion) {
	c.bench_function("Load as char", |b| {
		b.iter(|| {
			let x = load("words.txt".to_string());
		});
	});
}
fn genperms(c: &mut Criterion) {
	let x = &gen_color_perms();
	c.bench_function("Generate permutations", |b| {
		b.iter(|| {
			let mut copied_map = HashMap::new();
			// x.iter().for_each(|(key,_value)|{
			// 	let cloned_value = Mutex::new(0);
			// 	let cloned_key = key;
			// 	copied_map.insert(cloned_key, cloned_value);
			// });
			for (key, value) in x {
				let cloned_value = Mutex::new(0);
				let cloned_key = key;
				copied_map.insert(cloned_key, cloned_value);
				// let cloned_value = Mutex::new(*value.lock().unwrap());
				// let cloned_value = value.clone();
			}
		});
	});
}

fn calc_entropy(c: &mut Criterion) {
	let words = load("words.txt".to_string());
	let count = words.len();
	let perms = gen_color_perms();
	let input = words.get(1).unwrap().clone();
	c.bench_function("Calculate entropy mutex", |b| {
		b.iter(|| {
			words.par_iter().for_each(|f| {
				let real = f;
				let res = cmp_words(real, &input);
				if let Some(x) = perms.get(&res) {
					{
						*x.lock().unwrap() += 1;
					}
				}
			});
			let mut entropy = AtomicI32::new(0);
			perms.values().par_bridge().for_each(|f| {
				let v;
				{
					v = *f.lock().unwrap();
				}
				let p = v as f64 / count as f64;
				match p {
					0.0 => {}
					_ => {
						let a = (1.0 / p).log2();
						// entropy += a * p;
						entropy.fetch_add(
							(a * p * 100.0) as i32,
							std::sync::atomic::Ordering::Relaxed,
						);
					}
				}
			});
		});
	});
}
fn calc_entropy2(c: &mut Criterion) {
	let words = load("words.txt".to_string());
	let count = words.len();
	// let perms = gen_color_perms();
	let input = words.get(0).unwrap();
	c.bench_function("Calculate entropy atomic", |b| {
		b.iter(|| {
			let list = [const { AtomicI32::new(0 as i32) }; LEN as usize];
			words.par_iter().for_each(|f| {
				let real = f;
				let res = cmp_words(real, input);
				list[res_to_num(res) as usize].fetch_add(1, std::sync::atomic::Ordering::Relaxed);
				// list[res_to_num(res) as usize] += 1;
			});
			// let mut entropy = 0.0;

			let entropy = AtomicI32::new(0);
			list.par_iter().for_each(|f| {
				let p = f.fetch_add(0, std::sync::atomic::Ordering::Relaxed) as f64 / count as f64;
				match p {
					0.0 => {}
					_ => {
						let a = (1.0 / p).log2();
						// entropy += a * p;
						entropy.fetch_add(
							(a * p * 100.0) as i32,
							std::sync::atomic::Ordering::Relaxed,
						);
					}
				}
			});
		});
	});
}

fn calc_all_entropy(c: &mut Criterion) {
	let words = load("words.txt".to_string());
	let count = words.len();
	// let perms = gen_color_perms();
	// let input = words.get(0).unwrap();
	c.bench_function("Calc all entropy", |b| {
		b.iter(|| {
			words.par_iter().for_each(|input| {
				let list = [const { AtomicI32::new(0 as i32) }; LEN as usize];
				words.iter().for_each(|f| {
					let real = f;
					let res = cmp_words(real, input);
					list[res_to_num(res) as usize]
						.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
					// list[res_to_num(res) as usize] += 1;
				});
				// let mut entropy = 0.0;

				let entropy = AtomicI32::new(0);
				list.iter().for_each(|f| {
					let p =
						f.fetch_add(0, std::sync::atomic::Ordering::Relaxed) as f64 / count as f64;
					match p {
						0.0 => {}
						_ => {
							let a = (1.0 / p).log2();
							// entropy += a * p;
							entropy.fetch_add(
								(a * p * 100.0) as i32,
								std::sync::atomic::Ordering::Relaxed,
							);
						}
					}
				});
			});
		});
	});
}
fn cmp_char(c: &mut Criterion) {
	let x = load("words.txt".to_string());
	let a = x.get(1).unwrap();
	let z = x.get(2).unwrap();
	c.bench_function("Compare as char", |b| {
		b.iter(|| {
			cmp_words(a, z);
		});
	});
}
fn cmp_char_hash(c: &mut Criterion) {
	let x = load("words.txt".to_string());
	let a = x.get(1).unwrap();
	let z = x.get(2).unwrap();
	c.bench_function("Compare as char (hashed)", |b| {
		b.iter(|| {
			cmp_words_hash(a, z);
		});
	});
}
fn cmp_rand_char(c: &mut Criterion) {
	let x = load("words.txt".to_string());
	c.bench_function("Compare as random char", |b| {
		b.iter(|| {
			let a = x.get(rand::rng().random_range(0..MAX)).unwrap();
			let z = x.get(rand::rng().random_range(0..MAX)).unwrap();
			cmp_words(a, z);
		});
	});
}
fn cmp_rand_char_hash(c: &mut Criterion) {
	let x = load("words.txt".to_string());
	c.bench_function("Compare as random char (hashed)", |b| {
		b.iter(|| {
			let a = x.get(rand::rng().random_range(0..MAX)).unwrap();
			let z = x.get(rand::rng().random_range(0..MAX)).unwrap();
			cmp_words_hash(a, z);
		});
	});
}
fn cmp_rand_char_hash_small(c: &mut Criterion) {
	let x = load("words.txt".to_string());
	c.bench_function("Compare small random char (hashed)", |b| {
		b.iter(|| {
			let a = x.get(rand::rng().random_range(0..1000)).unwrap();
			let z = x.get(rand::rng().random_range(0..1000)).unwrap();
			cmp_words_hash(a, z);
		});
	});
}

fn entropy(c: &mut Criterion) {
	c.bench_function("Entropy", |b| {
		b.iter(|| {
			let p: f64 = random();
			let a = (1.0 / p).log2();
			let entropy = a * p;
		});
	});
}

fn cmpstr(c: &mut Criterion) {
	let x = load_str("words.txt".to_string());
	let a = x.get(1).unwrap();
	let z = x.get(2).unwrap();
	c.bench_function("Compare as str", |b| {
		b.iter(|| {
			cmp_words_str(a, z);
		});
	});
}

fn loadstr(c: &mut Criterion) {
	c.bench_function("Load as str", |b| {
		b.iter(|| {
			let x = load_str("words.txt".to_string());
		});
	});
}

// criterion_group!(benches, cmp_char, cmp_rand_char, cmp_rand_char_hash);
criterion_group!(
	benches,
	cmp_char,
	cmp_char_hash,
	cmp_rand_char,
	cmp_rand_char_hash,
	cmp_rand_char_hash_small,
);
criterion_main!(benches);
