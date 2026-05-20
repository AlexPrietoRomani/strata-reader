//! Additive-Increase, Multiplicative-Decrease (AIMD) back-pressure
//! controller — TCP-style congestion control adapted to the IA bridge.
//!
//! Plan Maestro §13.T8.3 — the Triage Engine fires crops at the Python
//! IA microservice as fast as the Rust scheduler can, but the GPU is the
//! real bottleneck. The controller tunes the concurrency window:
//!
//! - **AI (additive increase)** on success: `current += 1` each time a
//!   successful response lands, capped at `max_concurrency`.
//! - **MD (multiplicative decrease)** on a *resource* failure
//!   (`RESOURCE_EXHAUSTED`, timeout, OllamaUnreachable retry-exhausted):
//!   `current = max(min_concurrency, (current * factor) as usize)` with
//!   `factor ∈ (0,1)` (default 0.5 — classic TCP behaviour).
//!
//! Latency-based feedback is layered on top: if the p95 latency of the
//! last [`BackpressureConfig::window_size`] observations exceeds
//! `target_p95_ms`, the controller treats that as an MD trigger even
//! when the response was technically successful.
//!
//! The controller is `Sync` (interior mutability via [`parking_lot::Mutex`])
//! so the scheduler and the bridge client can both call it.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use parking_lot::Mutex;

#[derive(Clone, Debug)]
pub struct BackpressureConfig {
    pub min_concurrency: usize,
    pub max_concurrency: usize,
    /// Concurrency floor / ceiling are inclusive.
    pub initial_concurrency: usize,
    /// Below this, an MD trigger is a no-op (we're already at the floor).
    pub multiplicative_factor: f32,
    pub target_p95_ms: u32,
    pub window_size: usize,
}

impl Default for BackpressureConfig {
    fn default() -> Self {
        Self {
            min_concurrency: 1,
            max_concurrency: 32,
            initial_concurrency: 4,
            multiplicative_factor: 0.5,
            target_p95_ms: 30_000, // 30 s — typical IA crop budget.
            window_size: 32,
        }
    }
}

/// Reason a caller wants the controller to back off.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BackoffReason {
    /// gRPC RESOURCE_EXHAUSTED (Python `resource_guard.py` denied).
    ResourceExhausted,
    /// Timeout reached without a response.
    Timeout,
    /// IA server unreachable (connect error after all retries).
    Unreachable,
    /// p95 latency exceeded the configured target.
    LatencyBudgetExceeded,
}

pub struct BackpressureController {
    cfg: BackpressureConfig,
    current: AtomicUsize,
    window: Mutex<LatencyWindow>,
}

impl BackpressureController {
    pub fn new(cfg: BackpressureConfig) -> Arc<Self> {
        let initial = cfg
            .initial_concurrency
            .clamp(cfg.min_concurrency, cfg.max_concurrency);
        let window = LatencyWindow::new(cfg.window_size);
        Arc::new(Self {
            current: AtomicUsize::new(initial),
            cfg,
            window: Mutex::new(window),
        })
    }

    /// Current concurrency window. Read every time the scheduler picks
    /// up a new batch.
    pub fn current(&self) -> usize {
        self.current.load(Ordering::Relaxed)
    }

    /// Observe a successful request and its latency. Additive-increase
    /// unless the latency budget has been breached (then MD).
    pub fn on_success(&self, latency: Duration) {
        let ms = latency.as_millis() as u32;
        let mut win = self.window.lock();
        win.push(ms);
        let p95 = win.percentile(95);
        drop(win);

        if p95 > self.cfg.target_p95_ms {
            self.decrease(BackoffReason::LatencyBudgetExceeded);
        } else {
            self.increase();
        }
    }

    /// Observe a failure. Always MD (independent of latency).
    pub fn on_failure(&self, reason: BackoffReason) {
        self.decrease(reason);
    }

    fn increase(&self) {
        let max = self.cfg.max_concurrency;
        loop {
            let cur = self.current.load(Ordering::Relaxed);
            if cur >= max {
                return;
            }
            let next = cur + 1;
            if self
                .current
                .compare_exchange(cur, next, Ordering::Relaxed, Ordering::Relaxed)
                .is_ok()
            {
                tracing::debug!(
                    target = "strata.backpressure",
                    new = next,
                    "additive_increase"
                );
                return;
            }
        }
    }

    fn decrease(&self, reason: BackoffReason) {
        let min = self.cfg.min_concurrency;
        let factor = self.cfg.multiplicative_factor.clamp(0.05, 0.95);
        loop {
            let cur = self.current.load(Ordering::Relaxed);
            if cur <= min {
                return;
            }
            let next = ((cur as f32 * factor) as usize).max(min);
            if self
                .current
                .compare_exchange(cur, next, Ordering::Relaxed, Ordering::Relaxed)
                .is_ok()
            {
                tracing::warn!(
                    target = "strata.backpressure",
                    reason = ?reason,
                    from = cur,
                    to = next,
                    "multiplicative_decrease"
                );
                return;
            }
        }
    }

    pub fn config(&self) -> &BackpressureConfig {
        &self.cfg
    }
}

// ---------------------------------------------------------------------------
// Latency window (ring buffer of recent observations).
// ---------------------------------------------------------------------------

struct LatencyWindow {
    buf: Vec<u32>,
    cap: usize,
    next: usize,
    len: usize,
}

impl LatencyWindow {
    fn new(cap: usize) -> Self {
        Self {
            buf: vec![0u32; cap],
            cap,
            next: 0,
            len: 0,
        }
    }

    fn push(&mut self, value: u32) {
        self.buf[self.next] = value;
        self.next = (self.next + 1) % self.cap;
        if self.len < self.cap {
            self.len += 1;
        }
    }

    /// Inclusive percentile (`50` = median, `95` = p95). Uses linear
    /// interpolation on a sorted copy — fine for ≤ 1k entries.
    fn percentile(&self, p: u8) -> u32 {
        if self.len == 0 {
            return 0;
        }
        let mut sample = self.buf[..self.len].to_vec();
        sample.sort_unstable();
        let p = p.min(100) as f32 / 100.0;
        let pos = ((self.len as f32 - 1.0) * p).round() as usize;
        sample[pos]
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_concurrency_within_bounds() {
        let ctrl = BackpressureController::new(BackpressureConfig {
            min_concurrency: 2,
            max_concurrency: 10,
            initial_concurrency: 5,
            ..BackpressureConfig::default()
        });
        assert_eq!(ctrl.current(), 5);
    }

    #[test]
    fn initial_clamps_to_max() {
        let ctrl = BackpressureController::new(BackpressureConfig {
            min_concurrency: 1,
            max_concurrency: 4,
            initial_concurrency: 100,
            ..BackpressureConfig::default()
        });
        assert_eq!(ctrl.current(), 4);
    }

    #[test]
    fn on_success_increases_concurrency_additively() {
        let ctrl = BackpressureController::new(BackpressureConfig {
            min_concurrency: 1,
            max_concurrency: 8,
            initial_concurrency: 2,
            target_p95_ms: 60_000,
            ..BackpressureConfig::default()
        });
        for _ in 0..5 {
            ctrl.on_success(Duration::from_millis(100));
        }
        assert_eq!(ctrl.current(), 7); // 2 + 5
    }

    #[test]
    fn on_success_never_exceeds_max() {
        let ctrl = BackpressureController::new(BackpressureConfig {
            min_concurrency: 1,
            max_concurrency: 4,
            initial_concurrency: 3,
            target_p95_ms: 60_000,
            ..BackpressureConfig::default()
        });
        for _ in 0..20 {
            ctrl.on_success(Duration::from_millis(10));
        }
        assert_eq!(ctrl.current(), 4);
    }

    #[test]
    fn on_failure_halves_concurrency() {
        let ctrl = BackpressureController::new(BackpressureConfig {
            min_concurrency: 1,
            max_concurrency: 32,
            initial_concurrency: 16,
            multiplicative_factor: 0.5,
            ..BackpressureConfig::default()
        });
        ctrl.on_failure(BackoffReason::ResourceExhausted);
        assert_eq!(ctrl.current(), 8);
        ctrl.on_failure(BackoffReason::ResourceExhausted);
        assert_eq!(ctrl.current(), 4);
    }

    #[test]
    fn on_failure_never_drops_below_min() {
        let ctrl = BackpressureController::new(BackpressureConfig {
            min_concurrency: 2,
            max_concurrency: 32,
            initial_concurrency: 3,
            ..BackpressureConfig::default()
        });
        for _ in 0..10 {
            ctrl.on_failure(BackoffReason::Timeout);
        }
        assert_eq!(ctrl.current(), 2);
    }

    #[test]
    fn p95_breach_triggers_decrease_even_on_success() {
        let ctrl = BackpressureController::new(BackpressureConfig {
            min_concurrency: 1,
            max_concurrency: 32,
            initial_concurrency: 16,
            target_p95_ms: 100,
            window_size: 10,
            ..BackpressureConfig::default()
        });
        // First fill window with fast responses → AI bumps us up to 32.
        for _ in 0..10 {
            ctrl.on_success(Duration::from_millis(5));
        }
        let after_fast = ctrl.current();
        // Now push 10 slow responses → window p95 jumps above 100ms.
        for _ in 0..10 {
            ctrl.on_success(Duration::from_millis(500));
        }
        let after_slow = ctrl.current();
        assert!(
            after_slow < after_fast,
            "expected MD on p95 breach, {after_slow} < {after_fast}"
        );
    }

    #[test]
    fn percentile_computation_is_sane() {
        let mut w = LatencyWindow::new(5);
        for v in [10u32, 20, 30, 40, 50] {
            w.push(v);
        }
        assert_eq!(w.percentile(0), 10);
        assert_eq!(w.percentile(50), 30);
        assert_eq!(w.percentile(100), 50);
    }

    #[test]
    fn window_wraps_around_after_capacity() {
        let mut w = LatencyWindow::new(3);
        w.push(1);
        w.push(2);
        w.push(3);
        w.push(4); // overwrites position 0 (value 1).
                   // Buffer should hold {4, 2, 3}; median = 3.
        assert_eq!(w.percentile(50), 3);
        assert_eq!(w.percentile(100), 4);
    }
}
