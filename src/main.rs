use clap::Parser;
use rayon::prelude::*;
use std::{
	fs::{self, OpenOptions},
	sync::Mutex,
	time::Instant,
};
use wordle::*;

mod tests;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
	#[arg(short, long)]
	words: String,

	#[arg(short, long)]
	entropy: String,
}

fn main() {
	clearscreen::clear().expect("failed to clear screen");
	let args = Args::parse();
	let start = Instant::now();

	let words = load(args.words);
	let count = words.len();
	dbg!(&count);

	if let Ok(false) = fs::exists(&args.entropy) {
		let l1 = (0..count).map(|_f| Mutex::new(0.0)).collect::<Vec<_>>();
		words
			// .iter()
			.par_iter()
			.zip(
				(0..count)
					// .par_bridge()
					.collect::<Vec<_>>(),
			)
			.for_each(|(input, i)| {
				// let list = [const { AtomicI32::new(0 as i32) }; LEN as usize];
				let mut list = [0; LEN as usize];
				words.iter().for_each(|f| {
					let real = f;
					let res = cmp_words(real, input);
					list[res_to_num(res) as usize] += 1;
					// .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
					// list[res_to_num(res) as usize] += 1;
				});
				let mut entropy = 0.0;
				// let mut entropy = 0;
				list.iter().for_each(|&f| {
					// let p = f.load(std::sync::atomic::Ordering::Relaxed) as f64
					let p = f as f64 / count as f64;
					match p {
						0.0 => {}
						_ => {
							let a = (1.0 / p).log2();
							let e = a * p;
							entropy += e;
						}
					}
				});
				*l1[i].lock().unwrap() = entropy;
			});
		let l2: Vec<Option<f64>> = l1
			.par_iter()
			.map(|f| Some(*f.lock().unwrap()))
			.collect::<Vec<Option<_>>>();
		let mut file = OpenOptions::new()
			.create(true)
			// .truncate(true) // If the file already exists we want to overwrite the old data
			.write(true)
			.read(true)
			.open(&args.entropy)
			.unwrap();
		let _ = serde_json::to_writer(&mut file, &l2);

		let mut max = None;
		for &i in l2.iter() {
			if i > max {
				max = i;
			}
		}
		dbg!(max);
	}

	let lookup: Vec<Option<f64>> =
		serde_json::from_reader(OpenOptions::new().read(true).open(args.entropy).unwrap()).unwrap();

	#[derive(Debug)]
	struct Rex {
		choice: [char; SIZE],
		res: [Color; SIZE],
	}
	let mut c = 0;

	let mut results = vec![];
	// {let real = words.get(2314).unwrap();
	{
		let real = &convert1("among");
		// for real in words.iter().take(4) {
		// for real in &words {
		clearscreen::clear().expect("failed to clear screen");
		c += 1;
		// println!("{:?}", char_to_str(real));
		let mut l2 = lookup.clone();
		let mut choices = vec![];
		// choices.push(Rex{choice:convert1("raise"), res:[Color::Grey,Color::Grey,Color::Grey,Color::Grey,Color::Grey]});
		let maxtires = 10;
		for n in 1..=maxtires {
			if n >= maxtires {
				panic!();
			}

			let mut max = None;
			for &i in l2.iter() {
				if i > max {
					max = i;
				}
			}
			// dbg!(max);
			let i = l2.iter().position(|&f| f >= max).unwrap();
			let choice = *words.get(i).unwrap();
			// println!("{}", max.unwrap());
			let res = cmp_words_inp(real, &choice);
			// let res = cmp_words(real, &choice);
			println!(
				"{} {:?}",
				color_to_str(res),
				char_to_str(&choice),
				// max.unwrap()
			);
			if choice == *real || res == [Color::Green; SIZE] {
				results.push(n);
				break;
			}
			choices.push(Rex { choice, res });
			let l1 = (0..count).map(|_| Mutex::new(None)).collect::<Vec<_>>();
			words
				.par_iter()
				.zip((0..count).collect::<Vec<_>>())
				.for_each(|(current, i)| {
					for x in choices.iter() {
						if cmp_words(current, &x.choice) != x.res {
							return;
						}
					}
					let mut list = [0; LEN as usize];
					let mut total = 0;
					words.iter().for_each(|f| {
						let real = f;
						let res = cmp_words(real, current);

						let mut valid = true;
						for x in choices.iter() {
							if cmp_words(f, &x.choice) != x.res {
								valid = false;
								break;
							}
						}
						if valid {
							list[res_to_num(res) as usize] += 1;
							total += 1;
						}
					});
					let mut entropy = 0.0;
					list.iter().for_each(|&f| {
						let p = f as f64 / total as f64;
						match p {
							0.0 => {}
							_ => {
								let a = (1.0 / p).log2();
								let e = a * p;
								entropy += e;
							}
						}
					});
					*l1[i].lock().unwrap() = Some(entropy);
					// dbg!(total);
				});
			l2 = l1
				.par_iter()
				.map(|f| *f.lock().unwrap())
				.collect::<Vec<_>>();
		}
	}

	let mut max = 0;
	let mut min = 100;
	let mut total = 0;
	for &i in results.iter() {
		if i > max {
			max = i;
		}
		if i < min {
			min = i;
		}
		total += i;
	}
	dbg!(max);
	dbg!(min);
	dbg!(total as f64 / c as f64);

	let duration = start.elapsed();
	println!("Time elapsed is: {:?}", duration);
}
