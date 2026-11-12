#[path = "../src/stats.rs"]
mod stats;

use stats::LatencyStats;

#[test]
fn test_latency_statistics() {
    let mut s = LatencyStats::new();
    assert_eq!(s.count(), 0);
    assert!(s.mean().is_none());

    s.record(10.0);
    s.record(20.0);
    s.record(30.0);

    assert_eq!(s.count(), 3);
    assert_eq!(s.min(), Some(10.0));
    assert_eq!(s.max(), Some(30.0));
    assert_eq!(s.mean(), Some(20.0));
    assert_eq!(s.jitter(), Some(10.0));
    assert_eq!(s.percentile(50.0), Some(20.0));
}
