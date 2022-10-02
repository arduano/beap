use super::BeapCoordinate;

// A trait to track steps
// This is useful because we can insert a struct that ignores steps entirely
// or one that tracks and provides an array of them for animation
pub trait TrackSteps<T> {
    type WrapOutput<O>;

    fn new() -> Self;
    fn add_step(&mut self, step: T);
    fn wrap_output<O>(self, output: O) -> Self::WrapOutput<O>;
}

// Track steps
pub struct StepTracker<T> {
    steps: Vec<T>,
}

impl<T> StepTracker<T> {
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }
}

impl<T> TrackSteps<T> for StepTracker<T> {
    type WrapOutput<O> = ResultWithSteps<O, Vec<T>>;

    fn add_step(&mut self, step: T) {
        self.steps.push(step);
    }

    fn wrap_output<O>(self, output: O) -> Self::WrapOutput<O> {
        ResultWithSteps {
            result: output,
            steps: self.steps,
        }
    }

    fn new() -> Self {
        Self::new()
    }
}

// Ignore steps
pub struct IgnoreSteps;

impl<T> TrackSteps<T> for IgnoreSteps {
    type WrapOutput<O> = O;

    fn add_step(&mut self, _step: T) {}

    fn wrap_output<O>(self, output: O) -> Self::WrapOutput<O> {
        output
    }

    fn new() -> Self {
        IgnoreSteps
    }
}

pub struct ResultWithSteps<R, S> {
    pub result: R,
    pub steps: S,
}

// Step for animating modifications to the structure
// All modifications in a beap come as a set of swaps
pub struct AnimatedSwap {
    pub first: BeapCoordinate,
    pub second: BeapCoordinate,
    pub overwrite: bool,
}

// Step for animating modifications to the structure
// All modifications in a beap come as a set of swaps
#[derive(Debug)]
pub struct AnimatedSearch {
    pub coord: BeapCoordinate,
}
