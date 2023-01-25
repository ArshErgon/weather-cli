extern crate pbr;
use std::time;
use clearscreen;


use pbr::ProgressBar;
use std::thread;

pub fn spinner_loader() {
    clearscreen::clear().expect("failed to clear screen");

    let msg = format!("
        \n\t\t\t   Starting up Weather App 🚀\n
        \t\t\tVersion: 0.0.1\n
        \t\t\tMade In Dehradun\n
        \t\t\tMade In Rust with 💙\n
    ");
    println!("{msg}");
    thread::sleep(time::Duration::from_millis(20));

    let count = 100;
    let mut pb = ProgressBar::new(count);
    pb.format("---");
    for _ in 0..count {
        pb.inc();
        thread::sleep(time::Duration::from_millis(25));
    }
}
