use criterion::{criterion_group, criterion_main, Criterion, black_box};
use dywapitchtrack::DywaPitchTracker;

pub fn compute_pitch_benchmark(c: &mut Criterion) {
    const SAMPLE_RATE: usize = 44100;
    const SIZE: usize = 1024;

    let dt = 1.0 / SAMPLE_RATE as f32;
    let freq = 300.0;
    let samples: Vec<f32> = (0..SIZE)
        .map(|x| (2.0 * std::f32::consts::PI * x as f32 * dt * freq).sin())
        .collect();

    c.bench_function("DywaPitchTracker compute_pitch", |b| {
        let mut pitch_tracker = DywaPitchTracker::new();
        b.iter(|| pitch_tracker.compute_pitch(black_box(&samples), 0, SIZE));
    });
}

criterion_group!(benches, compute_pitch_benchmark);
criterion_main!(benches);
