#![feature(vec_remove_item, drain_filter)]
use rand::Rng;
use std::io;

fn new_boxes() -> [bool; 3] {
	let mut boxes = [false, false, false];
	boxes[rand::thread_rng().gen_range(0, 3)] = true;
	boxes
}

fn no_switch_box(tries: u64) -> (u64, u64) {
	let mut val = 0;
	let mut val_lose = 0;
	for _ in 0..tries {
		let boxes = new_boxes();
		let pick = boxes[rand::thread_rng().gen_range(0, 3)];
		if pick {
			val += 1;
		} else {
			val_lose += 1;
		}
	}
	(val, val_lose)
}

fn switch_box(tries: u64) -> (u64, u64) {
	let mut val = 0;
	let mut val_lose = 0;
	for _ in 0..tries {
		let boxes = new_boxes();
		let first_pick: (usize, bool) = {
			let pick_box = rand::thread_rng().gen_range(0, 3);
			(pick_box, boxes[pick_box])
		};
		if first_pick.1 {
			//If the inital box pick is the winning box, automatically lose since they're going to switch
			val_lose += 1;
			continue;
		} else {
			let mut options: Vec<usize> = vec![0, 1, 2];
			options.remove_item(&first_pick.0);
			options.drain_filter(|x| !boxes[*x]);
			//options = options.into_iter().filter(|x| holder[*x]).collect();
			if boxes[options[0]] {
				val += 1;
			}
		}
	}
	(val, val_lose)
}

fn main() {
	let tries = get_input();
	let (no_switch_wins, no_switch_losses) = no_switch_box(tries);
	let (switch_wins, switch_losses) = switch_box(tries);
	println!(
		"No switch wins {}/{}... {}:{}, {}% wins",
		no_switch_wins,
		no_switch_wins + no_switch_losses,
		no_switch_wins,
		no_switch_losses,
		(no_switch_wins as f64 / (no_switch_wins + no_switch_losses) as f64) * 100f64
	);
	println!(
		"Switch wins {}/{}... {}:{}, {}% wins",
		switch_wins,
		switch_wins + switch_losses,
		switch_wins,
		switch_losses,
		(switch_wins as f64 / (switch_wins + switch_losses) as f64) * 100f64
	);
	hang();
}

fn get_input() -> u64 {
	println!("Please input number of attempts");
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Line read error");
	input.trim().parse::<u64>().unwrap()
}

fn hang() {
	println!("Press enter to exit");
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Line read error");
}
