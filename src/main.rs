mod probe;
mod stats;

use clap::Parser;
use colored::*;
use probe::probe_socket;
use stats::LatencyStats;
use std::net::{SocketAddr, ToSocketAddrs};
use tokio::time::{sleep, Duration};

#[derive(Parser, Debug)]
#[command(author, version, about = "High-precision concurrent network latency & jitter telemetry probe")]
struct Args {
    #[arg(short, long, help = "Target host with port (e.g., 1.1.1.1:443 or google.com:80)")]
    target: String,

    #[arg(short, long, default_value_t = 10, help = "Number of probe packets to transmit")]
    count: usize,

    #[arg(short, long, default_value_t = 250, help = "Interval between probes in milliseconds")]
    interval: u64,

    #[arg(short = 'w', long, default_value_t = 2000, help = "Timeout in milliseconds per probe")]
    timeout: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("{}", "=".repeat(60).cyan());
    println!("{} {} {}", "⚡".yellow(), "HYPER-PROBE".bold().white(), "v0.1.0".dimmed());
    println!("{} High-Resolution Network Telemetry Engine", "├─".cyan());
    println!("{} Author: Ismeet Pandey", "└─".cyan());
    println!("{}", "=".repeat(60).cyan());

    let addr: SocketAddr = match args.target.to_socket_addrs() {
        Ok(mut addrs) => match addrs.next() {
            Some(a) => a,
            None => {
                eprintln!("{}: unable to resolve address {}", "ERROR".red().bold(), args.target);
                std::process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("{}: {} ({})", "ERROR".red().bold(), args.target, e);
            std::process::exit(1);
        }
    };

    println!("{} Probing {} ({}) with {} samples...", "🎯".bold(), args.target.green(), addr, args.count);
    println!();

    let mut stats = LatencyStats::new();
    let mut lost = 0;

    for i in 1..=args.count {
        let res = probe_socket(i, addr, args.timeout).await;
        if res.success {
            let lat = res.latency_ms.unwrap();
            stats.record(lat);
            let lat_str = format!("{:.2} ms", lat);
            let colored_lat = if lat < 30.0 {
                lat_str.green()
            } else if lat < 100.0 {
                lat_str.yellow()
            } else {
                lat_str.red()
            };
            println!("  [{:>3}/{}] {} -> latency = {}", i, args.count, "PASS".green().bold(), colored_lat);
        } else {
            lost += 1;
            println!("  [{:>3}/{}] {} -> {}", i, args.count, "FAIL".red().bold(), res.error.unwrap_or_default().dimmed());
        }

        if i < args.count {
            sleep(Duration::from_millis(args.interval)).await;
        }
    }

    println!();
    println!("{}", "─── TELEMETRY SUMMARY ──────────────────────────────────────".cyan());
    let sent = args.count;
    let recv = stats.count();
    let loss_pct = (lost as f64 / sent as f64) * 100.0;
    println!("  Packets: Transmitted = {}, Received = {}, Loss = {:.1}%", sent, recv, loss_pct);

    if let (Some(min), Some(max), Some(avg), Some(jit)) = (stats.min(), stats.max(), stats.mean(), stats.jitter()) {
        println!("  Latency: Min = {:.2}ms | Avg = {:.2}ms | Max = {:.2}ms", min, avg, max);
        println!("  Jitter:  {:.2}ms (mean consecutive sample variance)", jit);
        println!("  Percentiles: P50 = {:.2}ms | P90 = {:.2}ms | P99 = {:.2}ms", 
            stats.percentile(50.0).unwrap_or(avg),
            stats.percentile(90.0).unwrap_or(max),
            stats.percentile(99.0).unwrap_or(max)
        );
    }
    println!("{}", "────────────────────────────────────────────────────────────".cyan());

    Ok(())
}
