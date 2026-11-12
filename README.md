# ⚡ hyper-probe

<div align="center">

[![Rust CI](https://github.com/IsmeetPandey/hyper-probe/actions/workflows/ci.yml/badge.svg)](https://github.com/IsmeetPandey/hyper-probe/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Language: Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-red.svg)](https://crates.io/)

**Blazing-fast, concurrent TCP & HTTP latency probe with percentile distribution (P50/P90/P99) and jitter analysis.**

</div>

---

## 🚀 Features

- **⚡ Sub-Millisecond Precision**: Built on top of `tokio` async networking runtime for zero-overhead timing.
- **📊 Statistical Telemetry**: Automated calculation of Min, Mean, Max, Jitter, and P50/P90/P99 percentiles.
- **🎨 Rich ANSI Terminal Output**: Dynamic color-coded thresholds (green < 30ms, yellow < 100ms, red >= 100ms).
- **🛡️ Resilient Connection Handling**: Configurable per-probe timeouts, intervals, and socket resolution.

---

## 📦 Installation

### From Source
```bash
git clone https://github.com/IsmeetPandey/hyper-probe.git
cd hyper-probe
cargo build --release
./target/release/hyper-probe --help
```

---

## 💻 Usage

Probe an endpoint (e.g. Cloudflare DNS or Google HTTP):

```bash
hyper-probe --target 1.1.1.1:443 --count 15 --interval 200
```

### Example Output

```text
============================================================
⚡ HYPER-PROBE v0.1.0
├─ High-Resolution Network Telemetry Engine
└─ Author: Ismeet Pandey
============================================================
🎯 Probing 1.1.1.1:443 (1.1.1.1:443) with 10 samples...

  [  1/10] PASS -> latency = 14.23 ms
  [  2/10] PASS -> latency = 12.87 ms
  [  3/10] PASS -> latency = 13.04 ms
  [  4/10] PASS -> latency = 13.51 ms
  [  5/10] PASS -> latency = 12.92 ms
  [  6/10] PASS -> latency = 14.11 ms
  [  7/10] PASS -> latency = 13.78 ms
  [  8/10] PASS -> latency = 12.65 ms
  [  9/10] PASS -> latency = 13.44 ms
  [ 10/10] PASS -> latency = 13.19 ms

─── TELEMETRY SUMMARY ──────────────────────────────────────
  Packets: Transmitted = 10, Received = 10, Loss = 0.0%
  Latency: Min = 12.65ms | Avg = 13.37ms | Max = 14.23ms
  Jitter:  0.48ms (mean consecutive sample variance)
  Percentiles: P50 = 13.32ms | P90 = 14.12ms | P99 = 14.22ms
────────────────────────────────────────────────────────────
```

---

## 🛠️ CLI Options

| Option | Short | Default | Description |
|---|---|---|---|
| `--target` | `-t` | *(Required)* | Target host and port (`host:port`) |
| `--count` | `-c` | `10` | Number of probe packets to transmit |
| `--interval` | `-i` | `250` | Milliseconds to sleep between probes |
| `--timeout` | `-w` | `2000` | Milliseconds before a probe connection times out |

---

## 📄 License

MIT License © 2026 [Ismeet Pandey](https://github.com/IsmeetPandey).
