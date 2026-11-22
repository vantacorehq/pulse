//! pulse — pure Rust live TUI system monitor.
//! Cyberpunk radar style. Zero dependencies.

use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut stdout = io::stdout();
    let start = Instant::now();
    let mut frame = 0u64;

    loop {
        frame += 1;
        // clear + home
        print!("\x1b[2J\x1b[H");

        println!("\x1b[1;35m██ PULSE ██\x1b[0m  \x1b[90mcyberpunk system radar\x1b[0m");
        println!("\x1b[90m──────────────────────────────────────\x1b[0m");

        // Fake but smooth radar values (replace with /proc reads)
        let t = start.elapsed().as_secs_f32();
        let cpu = ((t.sin() * 0.4 + 0.5) * 100.0) as u32;
        let mem = 42 + (frame % 17) as u32;
        let disk = (frame % 9) as u32;
        let net = ((t * 3.0).sin().abs() * 80.0) as u32;

        draw_bar("CPU ", cpu, "\x1b[1;32m");
        draw_bar("RAM ", mem, "\x1b[1;36m");
        draw_bar("DISK", disk, "\x1b[1;33m");
        draw_bar("NET ", net, "\x1b[1;35m");

        println!();
        println!("\x1b[90mradar sweep: {}\x1b[0m", "░".repeat((frame % 20) as usize));
        println!("\x1b[90mCtrl-C to exit\x1b[0m");

        stdout.flush().ok();
        thread::sleep(Duration::from_millis(120));
    }
}

fn draw_bar(label: &str, pct: u32, color: &str) {
    let width = 30;
    let filled = (pct as usize * width / 100).min(width);
    let bar = "█".repeat(filled) + &"░".repeat(width - filled);
    println!("  {} {:<4} {}{}\x1b[0m  {}%", color, label, color, bar, pct);
}
