use std::net::SocketAddr;
use std::time::Instant;
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};

#[derive(Debug, Clone)]
pub struct ProbeResult {
    pub seq: usize,
    pub target: SocketAddr,
    pub latency_ms: Option<f64>,
    pub success: bool,
    pub error: Option<String>,
}

pub async fn probe_socket(seq: usize, addr: SocketAddr, timeout_ms: u64) -> ProbeResult {
    let start = Instant::now();
    let to = Duration::from_millis(timeout_ms);

    match timeout(to, TcpStream::connect(&addr)).await {
        Ok(Ok(_stream)) => {
            let elapsed = start.elapsed().as_secs_f64() * 1000.0;
            ProbeResult {
                seq,
                target: addr,
                latency_ms: Some(elapsed),
                success: true,
                error: None,
            }
        }
        Ok(Err(e)) => ProbeResult {
            seq,
            target: addr,
            latency_ms: None,
            success: false,
            error: Some(e.to_string()),
        },
        Err(_) => ProbeResult {
            seq,
            target: addr,
            latency_ms: None,
            success: false,
            error: Some("Connection timed out".to_string()),
        },
    }
}
