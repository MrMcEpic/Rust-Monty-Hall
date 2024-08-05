use rand::Rng;

fn new_boxes() -> [bool; 3] {
    //generates a list of 3 boxes all as false, then turns a random one true
    let mut boxes = [false; 3];
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
            options.retain(|&x| x != first_pick.0); // removes their initial box pick from the list of possible boxes

            if let Some(index) = options.iter().position(|&x| !boxes[x]) {
                options.remove(index); // removes the first false box
            }

            if boxes[options[0]] {
                val += 1;
            }
        }
    }
    (val, val_lose)
}
