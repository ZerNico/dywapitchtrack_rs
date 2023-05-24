[![crates.io](https://img.shields.io/crates/v/dywapitchtrack.svg)](https://crates.io/crates/dywapitchtrack)

# dywapitchtrack

## Usage

```rust
use dywapitchtrack::DywaPitchTracker;

fn main() {
    const SAMPLE_RATE: usize = 44100;
    const SIZE: usize = 1024;

    let dt = 1.0 / SAMPLE_RATE as f32;
    let freq = 300.0;

    // Sound samples
    let samples: Vec<f32> = (0..SIZE)
        .map(|x| (2.0 * std::f32::consts::PI * x as f32 * dt * freq).sin())
        .collect();

    let mut pitch_tracker = DywaPitchTracker::new();
    let pitch = pitch_tracker.compute_pitch(&samples, 0, SIZE)

    println!("Frequency: {}", pitch);
}
```