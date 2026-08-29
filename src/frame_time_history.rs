use macroquad::prelude::*;
use std::collections::VecDeque;

/// Number of samples to keep in the frame time history. IE, the number of frames to keep track of
/// for the rolling graph.
pub const HISTORY_LEN: usize = 120;

/// A fixed-size rolling buffer of recent frame times, in milliseconds.
pub struct FrameTimeHistory {
    samples: VecDeque<f64>,
}

/// Implement methods for the FrameTimeHistory struct.
impl FrameTimeHistory {
    /// Creates a new FrameTimeHistory with a fixed capacity.
    pub fn new() -> Self {
        Self {
            samples: VecDeque::with_capacity(HISTORY_LEN),
        }
    }

    /// Pushes a new sample, evicting the oldest once the buffer is full.
    pub fn push(&mut self, frame_time_ms: f64) {
        if self.samples.len() == HISTORY_LEN {
            self.samples.pop_front();
        }

        self.samples.push_back(frame_time_ms);
    }

    /// Returns the number of samples currently stored in the history.
    fn len(&self) -> usize {
        self.samples.len()
    }

    /// Returns a vector of references to the samples in the history.
    fn samples(&self) -> Vec<&f64> {
        self.samples.iter().collect()
    }

    /// Returns the average frame time in milliseconds. If there are no samples, returns 0.0.
    fn avg(&self) -> f64 {
        if self.samples.is_empty() {
            return 0.0;
        }

        self.samples.iter().copied().sum::<f64>() / self.samples.len() as f64
    }
}

/// Draws a rolling frame-time graph in the rect at (x, y) sized (w, h), with
/// 60fps/30fps reference lines. Scale floors at 33.3ms so ordinary jitter
/// doesn't make the graph rescale every frame.
pub fn draw(history: &FrameTimeHistory, x: f32, y: f32, w: f32, h: f32) {
    draw_rectangle(x, y, w, h, Color::new(0.0, 0.0, 0.0, 0.6));

    if history.len() < 2 {
        return;
    }

    let pad = 8.0;
    let inner_w = w - pad * 2.0;
    let inner_h = h - pad * 2.0;

    let max_ms = history.samples.iter().cloned().fold(33.3_f64, f64::max);
    let y_for = |ms: f64| -> f32 {
        let t = (ms / max_ms).clamp(0.0, 1.0) as f32;
        y + pad + inner_h - t * inner_h
    };

    // 16.6ms reference line (60fps).
    draw_line(x + pad, y_for(16.6), x + w - pad, y_for(16.6), 1.0, BLUE);

    // 33.3ms reference line (30fps).
    draw_line(x + pad, y_for(33.3), x + w - pad, y_for(33.3), 1.0, RED);

    let samples: Vec<&f64> = history.samples();

    // Horizontal distance between samples.
    let step = inner_w / (samples.len() as f32 - 1.0);

    // For each pair of samples, draw a line between them.
    for (i, pair) in samples.windows(2).enumerate() {
        let x0 = x + pad + i as f32 * step;
        let x1 = x + pad + (i + 1) as f32 * step;
        draw_line(x0, y_for(*pair[0]), x1, y_for(*pair[1]), 2.0, GREEN);
    }

    draw_text(
        format!(
            "frame time  (scale max {:.1}ms) (avg {:.1}ms)",
            max_ms,
            history.avg()
        ),
        x + 8.0,
        y + 20.0,
        18.0,
        WHITE,
    );
}
