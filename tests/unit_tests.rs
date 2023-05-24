#[cfg(test)]
mod tests {
    use std::path::Path;

    use dywapitchtrack::DywaPitchTracker;
    use hound::WavReader;


    // Setup function
    fn setup(file: &str) -> Vec<f32> {
        // Read the WAV file
        let file_path = Path::new(file);
        let reader = WavReader::open(&file_path).unwrap();
        let samples: Vec<f32> = reader.into_samples::<i32>().map(|s| s.unwrap() as f32).collect();

        samples
    }

    #[test]
    fn test_compute_pitch_250hz() {
      let samples = setup("tests/files/250hz.wav");
      let mut pitch_tracker = DywaPitchTracker::new();

      let result = pitch_tracker.compute_pitch(&samples, 0, samples.len());

      println!("Pitch: {}", result);

      // check if difference is less than 0.1 hz
      assert!((result - 250.0).abs() < 0.1);
    }

    #[test]
    fn test_compute_pitch_440hz() {
      let samples = setup("tests/files/440hz.wav");
      let mut pitch_tracker = DywaPitchTracker::new();

      let result = pitch_tracker.compute_pitch(&samples, 0, samples.len());

      println!("Pitch: {}", result);

      // check if difference is less than 0.1 hz
      assert!((result - 440.0).abs() < 0.1);
    }

    #[test]
    fn test_compute_pitch_silent() {
      let samples = setup("tests/files/silent.wav");
      let mut pitch_tracker = DywaPitchTracker::new();

      let result = pitch_tracker.compute_pitch(&samples, 0, samples.len());

      println!("Pitch: {}", result);

      // check if difference is less than 0.1 hz
      assert!((result - 0.0).abs() < 0.1);
    }
}
