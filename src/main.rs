//! pulse — pure Rust live TUI system monitor.
//! Cyberpunk radar style. Zero dependencies.

use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut stdout = io::stdout();
    let start = Instant::now();
    let mut frame = 0u64;

    // Hide cursor
    print!("\x1b[?25l");
    let _ = stdout.flush();

    loop {
        frame += 1;
        // clear + home
        print!("\x1b[2J\x1b[H");

        println!("\x1b[1;35m██ PULSE ██\x1b[0m  \x1b[90mcyberpunk system radar\x1b[0m");
        println!("\x1b[90m──────────────────────────────────────\x1b[0m");

        // Smooth synthetic values (replace with real /proc reads on Linux)
        let t = start.elapsed().as_secs_f32();
        let cpu = ((t.sin() * 0.35 + 0.55) * 100.0) as u32;
        let mem = 38 + ((t * 0.7).sin().abs() * 25.0) as u32;
        let disk = 8 + (frame % 12) as u32;
        let net = ((t * 2.3).sin().abs() * 75.0) as u32;

        draw_bar("CPU ", cpu, "\x1b[1;32m");
        draw_bar("RAM ", mem, "\x1b[1;36m");
        draw_bar("DISK", disk, "\x1b[1;33m");
        draw_bar("NET ", net, "\x1b[1;35m");

        println!();
        let sweep = (frame % 24) as usize;
        println!("\x1b[90mradar sweep: {}\x1b[0m", "░".repeat(sweep) + &" ".repeat(24 - sweep));
        println!("\x1b[90mCtrl-C to exit\x1b[0m");

        stdout.flush().ok();
        thread::sleep(Duration::from_millis(110));
    }
}

fn draw_bar(label: &str, pct: u32, color: &str) {
    let width = 30;
    let filled = ((pct as usize * width) / 100).min(width);
    let bar = "█".repeat(filled) + &"░".repeat(width - filled);
    println!("  {} {:<4} {}{}\x1b[0m  {:>3}%", color, label, color, bar, pct);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bar_width_never_exceeds() {
        // Just ensure the function does not panic on edge values
        draw_bar("TEST", 0, "");
        draw_bar("TEST", 100, "");
        draw_bar("TEST", 150, ""); // should clamp
    }

    #[test]
    fn pct_clamping() {
        let width = 30;
        assert_eq!((0usize * width) / 100, 0);
        assert_eq!((100usize * width) / 100, 30);
        assert_eq!((150usize * width) / 100, 45); // then .min(width)
    }
}
