#![feature(vec_remove_item)]
#![feature(drain_filter)]
use rand::Rng;
use std::io;

pub struct Boxes {
	box_1: bool,
	box_2: bool,
	box_3: bool,
}

impl Boxes {
	fn new() -> Boxes {
		let boxes = Boxes {
			box_1: false,
			box_2: false,
			box_3: false,
		};
		match rand::thread_rng().gen_range(0, 3) {
			0 => Boxes {
				box_1: true,
				..boxes
			},
			1 => Boxes {
				box_2: true,
				..boxes
			},
			2 => Boxes {
				box_3: true,
				..boxes
			},
			_ => panic!("Error gening box"),
		}
	}
}

fn no_switch_box(tries: u32) -> (u64, u64) {
	let mut val = 0;
	let mut val_lose = 0;
	for _ in 0..tries {
		let boxes = Boxes::new();
		let pick = {
			match rand::thread_rng().gen_range(0, 3) {
				0 => boxes.box_1,
				1 => boxes.box_2,
				2 => boxes.box_3,
				_ => panic!("Error picking box on no switch"),
			}
		};
		if pick {
			val += 1;
		} else {
			val_lose += 1;
		}
	}
	(val, val_lose)
}

fn switch_box(tries: u32) -> (u64, u64) {
	let mut val = 0;
	let mut val_lose = 0;
	for _ in 0..tries {
		let boxes = Boxes::new();
		let first_pick: (usize, bool) = {
			let pick_box = rand::thread_rng().gen_range(0, 3);
			(
				pick_box,
				match pick_box {
					0 => boxes.box_1,
					1 => boxes.box_2,
					2 => boxes.box_3,
					_ => panic!("Error picking box on switch"),
				},
			)
		};
		if first_pick.1 {
			//If the inital box pick is the winning box, automatically lose since they're going to switch
			val_lose += 1;
			continue;
		} else {
			let holder = [boxes.box_1, boxes.box_2, boxes.box_3]; //Holds respective true/false values for each box
			let mut options: Vec<usize> = vec![0, 1, 2];
			options.remove_item(&first_pick.0);
			options.drain_filter(|x| !holder[*x]);
			//options = options.into_iter().filter(|x| holder[*x]).collect();
			if holder[options[0]] {
				val += 1;
			}
		}
	}
	(val, val_lose)
}

fn main() {
	let tries = get_input();
	let no_switch_ratio = no_switch_box(tries);
	let switch_ratio = switch_box(tries);
	println!(
		"No switch wins {}/{}... {}:{}, {}% wins",
		no_switch_ratio.0,
		no_switch_ratio.0 + no_switch_ratio.1,
		no_switch_ratio.0,
		no_switch_ratio.1,
		(no_switch_ratio.0 as f64 / (no_switch_ratio.0 + no_switch_ratio.1) as f64) * 100f64
	);
	println!(
		"Switch wins {}/{}... {}:{}, {}% wins",
		switch_ratio.0,
		switch_ratio.0 + switch_ratio.1,
		switch_ratio.0,
		switch_ratio.1,
		(switch_ratio.0 as f64 / (switch_ratio.0 + switch_ratio.1) as f64) * 100f64
	);
	hang();
}

fn get_input() -> u32 {
	println!("Please input number of attempts");
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Line read error");
	input.trim().parse::<u32>().unwrap()
}

fn hang() {
	println!("Press enter to exit");
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Line read error");
}
