# pulse

Live TUI system monitor.

Pure Rust. Zero dependencies. Cyberpunk radar style.

## Why

No htop, no crates, no bloat. Just a binary that paints CPU, memory, disk and network in the terminal.

## Run

```sh
cargo run --release
# or
pulse
```

Press Ctrl-C to exit.

## What it shows

- CPU load (from /proc/stat on Linux, placeholder elsewhere)
- Memory used / total
- Disk activity
- Network connections / throughput
- Animated radar bars

## License

MIT
