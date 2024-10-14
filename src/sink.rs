use core::{fmt::Debug, num::NonZeroU32};

use crate::{samp::Sample, Frame};

/// Audio sink - a type that consumes audio samples.
pub trait Sink<Samp: Sample, const COUNT: usize>: Debug {
    /// Get the sample rate of the sink in hertz.
    fn sample_rate(&self) -> NonZeroU32;

    /// Get the length of the sink in frames.
    ///
    /// Sinks must always have finite length.
    fn len(&self) -> usize;

    /// Sink audio samples from a frame iterator.
    ///
    /// **Warning**: if used incorrectly, this method may introduce audio
    /// aliasing.  To avoid that, make sure the sample rate of the frames from
    /// the iterator matches exactly the sample rate of the sink.
    fn sink_with(&mut self, iter: &mut dyn Iterator<Item = Frame<Samp, COUNT>>);

    /// Check if the sink is empty (length of zero).
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Sink that converts to a different audio format before passing to another
/// [`Sink`](crate::Sink).
#[derive(Debug)]
pub struct SinkTo<Samp, S, K, const COUNT: usize, const N: usize>
where
    Samp: Sample + From<S>,
    S: Sample,
    K: Sink<Samp, COUNT>,
{
    sink: K,
    _phantom: core::marker::PhantomData<fn() -> (Samp, S)>,
}

impl<Samp, S, K, const COUNT: usize, const N: usize> SinkTo<Samp, S, K, COUNT, N>
where
    Samp: Sample + From<S>,
    S: Sample,
    K: Sink<Samp, COUNT>,
{
    /// Convert an arbitrary `Sink` type to a different format.
    pub fn new(sink: K) -> Self {
        Self {
            sink,
            _phantom: core::marker::PhantomData,
        }
    }
}

#[allow(single_use_lifetimes)]
impl<Samp, S, K, const COUNT: usize, const N: usize> Sink<S, N>
    for &mut SinkTo<Samp, S, K, COUNT, N>
where
    Samp: Sample + From<S>,
    S: Sample,
    K: Sink<Samp, COUNT>,
{
    /// Get the sample rate of the sink in hertz.
    fn sample_rate(&self) -> NonZeroU32 {
        self.sink.sample_rate()
    }

    /// Get the length of the sink in frames.
    ///
    /// Sinks must always have finite length.
    fn len(&self) -> usize {
        self.sink.len()
    }

    /// Sink audio samples from a frame iterator.
    ///
    /// **Warning**: if used incorrectly, this method may introduce audio
    /// aliasing.  To avoid that, make sure the sample rate of the frames from
    /// the iterator matches exactly the sample rate of the sink.
    fn sink_with(&mut self, iter: &mut dyn Iterator<Item = Frame<S, N>>) {
        self.sink.sink_with(&mut iter.map(Frame::to))
    }
}

#[allow(single_use_lifetimes)]
impl<Samp, S, K, const COUNT: usize, const N: usize> Sink<S, N>
    for SinkTo<Samp, S, K, COUNT, N>
where
    Samp: Sample + From<S>,
    S: Sample,
    K: Sink<Samp, COUNT>,
{
    /// Get the sample rate of the sink in hertz.
    fn sample_rate(&self) -> NonZeroU32 {
        self.sink.sample_rate()
    }

    /// Get the length of the sink in frames.
    ///
    /// Sinks must always have finite length.
    fn len(&self) -> usize {
        self.sink.len()
    }

    /// Sink audio samples from a frame iterator.
    ///
    /// **Warning**: if used incorrectly, this method may introduce audio
    /// aliasing.  To avoid that, make sure the sample rate of the frames from
    /// the iterator matches exactly the sample rate of the sink.
    fn sink_with(&mut self, iter: &mut dyn Iterator<Item = Frame<S, N>>) {
        self.sink.sink_with(&mut iter.map(Frame::to))
    }
}
