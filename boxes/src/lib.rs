#![feature(drain_filter)]
use rand::Rng;
//TODO Set up switch2

fn new_boxes() -> [bool; 3] {
    //generates a list of 3 boxes all as false, then turns a random one true
    let mut boxes = [false, false, false];
    boxes[rand::thread_rng().gen_range(0, 3)] = true;
    boxes
}

fn get_box(tries: u64) -> (u64, u64, u64, u64) {
    let mut no_win = 0;
    let mut no_loss = 0;
    let mut win = 0;
    let mut loss = 0;
    for _ in 0..tries {
        let boxes = new_boxes();
        let pick = boxes[rand::thread_rng().gen_range(0, 3)];
        if no_switch_box2(pick) {
            no_win += 1;
        } else {
            no_loss += 1;
        }
    }
    (win, loss, no_win, no_loss)
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

fn no_switch_box2(pick: bool) -> bool {
    if pick {
        return true;
    } else {
        return false;
    }
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
            options.remove(first_pick.0); //removes their inital box pick from a list of possible boxes
            options.drain_filter(|x| !boxes[*x]); //removes a false box from list of possible boxes
                                                  //options = options.into_iter().filter(|x| boxes[*x]).collect(); //other possible solution
            if boxes[options[0]] {
                val += 1;
            }
        }
    }
    (val, val_lose)
}
