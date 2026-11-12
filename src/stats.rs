#[derive(Debug, Clone, Default)]
pub struct LatencyStats {
    pub samples: Vec<f64>,
}

impl LatencyStats {
    pub fn new() -> Self {
        Self { samples: Vec::new() }
    }

    pub fn record(&mut self, latency_ms: f64) {
        self.samples.push(latency_ms);
    }

    pub fn count(&self) -> usize {
        self.samples.len()
    }

    pub fn min(&self) -> Option<f64> {
        self.samples.iter().cloned().reduce(f64::min)
    }

    pub fn max(&self) -> Option<f64> {
        self.samples.iter().cloned().reduce(f64::max)
    }

    pub fn mean(&self) -> Option<f64> {
        if self.samples.is_empty() {
            return None;
        }
        Some(self.samples.iter().sum::<f64>() / self.samples.len() as f64)
    }

    pub fn jitter(&self) -> Option<f64> {
        if self.samples.len() < 2 {
            return None;
        }
        let diffs: f64 = self
            .samples
            .windows(2)
            .map(|w| (w[1] - w[0]).abs())
            .sum();
        Some(diffs / (self.samples.len() - 1) as f64)
    }

    pub fn percentile(&self, p: f64) -> Option<f64> {
        if self.samples.is_empty() || p < 0.0 || p > 100.0 {
            return None;
        }
        let mut sorted = self.samples.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let rank = (p / 100.0) * (sorted.len() - 1) as f64;
        let lower = rank.floor() as usize;
        let upper = rank.ceil() as usize;
        if lower == upper {
            Some(sorted[lower])
        } else {
            let weight = rank - lower as f64;
            Some(sorted[lower] * (1.0 - weight) + sorted[upper] * weight)
        }
    }
}
