# pulse

**Live TUI system monitor in pure Rust.**

CPU · RAM · Disk · Network — cyberpunk radar style.

Zero dependencies. One binary. No crates.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

---

## Why it exists

Most system monitors pull in 30+ crates or force you into a full TUI framework.

`pulse` is intentionally minimal:

- only `std`
- smooth animated bars
- works on Linux (reads `/proc` where possible)
- falls back gracefully elsewhere

Perfect for servers, containers, and developer machines where you want something lighter than `htop`/`btop`.

## Install & run

```bash
cargo install --git https://github.com/vantacorehq/pulse
pulse
```

or

```bash
git clone https://github.com/vantacorehq/pulse
cd pulse
cargo run --release
```

Press `Ctrl-C` to exit.

## What you see

```
██ PULSE ██  cyberpunk system radar
──────────────────────────────────────
  CPU  ████████████████░░░░░░░░░░░░░░  52%
  RAM  ████████████░░░░░░░░░░░░░░░░░░  41%
  DISK ████░░░░░░░░░░░░░░░░░░░░░░░░░░  13%
  NET  ████████████████████░░░░░░░░░░  67%

radar sweep: ░░░░░░░░░░
```

## Code philosophy

No external TUI libraries. Everything is ANSI escape sequences + a simple loop.
Easy to fork and embed into your own tooling.

## Tests

```bash
cargo test
```

## Commercial

Need custom metrics, remote push, or a multi-node dashboard version?

Typical engagement: **$600 – $1 800**.

## License

MIT
