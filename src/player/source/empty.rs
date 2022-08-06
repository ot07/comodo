use std::marker::PhantomData;
use std::time::Duration;

use super::{Sample, Source};

/// An empty source.
#[derive(Debug, Copy, Clone)]
pub struct Empty<S>(PhantomData<S>);

impl<S> Default for Empty<S> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[allow(clippy::use_self)]
impl<S> Empty<S> {
    #[inline]
    pub const fn new() -> Empty<S> {
        Empty(PhantomData)
    }
}

impl<S> Iterator for Empty<S> {
    type Item = S;

    #[inline]
    fn next(&mut self) -> Option<S> {
        None
    }
}

impl<S> Source for Empty<S>
where
    S: Sample,
{
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        48000
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::new(0, 0))
    }

    #[inline]
    fn elapsed(&mut self) -> Duration {
        Duration::from_secs(0)
    }
    fn seek(&mut self, time: Duration) -> Option<Duration> {
        Some(time)
    }
}
