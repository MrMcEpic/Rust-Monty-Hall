use boxes::{no_switch_box, switch_box};
use std::io;

fn get_input() -> u64 {
    println!("Please input number of attempts");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Line read error");
    input.trim().parse::<u64>().unwrap()
}

#[allow(clippy::cast_precision_loss)]
fn print_out(wins: u64, losses: u64, switch: bool) {
    println!(
        "{} wins {}/{}... {}:{}, {}% wins",
        if switch { "Switch" } else { "No Switch" },
        wins,
        wins + losses,
        wins,
        losses,
        (wins as f64 / (wins + losses) as f64) * 100f64
    );
}

fn hang() {
    println!("Press enter to exit");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Line read error");
}

fn main() {
    let tries = get_input();
    let (no_switch_wins, no_switch_losses) = no_switch_box(tries);
    let (switch_wins, switch_losses) = switch_box(tries);
    print_out(no_switch_wins, no_switch_losses, false);
    print_out(switch_wins, switch_losses, true);
    hang();
}
