#![feature(vec_remove_item, drain_filter)]
use rand::Rng;

fn new_boxes() -> [bool; 3] {
	let mut boxes = [false, false, false];
	boxes[rand::thread_rng().gen_range(0, 3)] = true;
	boxes
}

pub fn no_switch_box(tries: u64) -> (u64, u64) {
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

pub fn switch_box(tries: u64) -> (u64, u64) {
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
			//options = options.into_iter().filter(|x| boxes[*x]).collect();
			if boxes[options[0]] {
				val += 1;
			}
		}
	}
	(val, val_lose)
}
