use core::{fmt::Debug, num::NonZeroU32};

use crate::{chan::Channel, Frame};

/// Audio sink - a type that consumes audio samples.
pub trait Sink<Chan: Channel, const CH: usize>: Debug {
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
    fn sink_with(&mut self, iter: &mut dyn Iterator<Item = Frame<Chan, CH>>);

    /// Check if the sink is empty (length of zero).
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Sink that converts to a different audio format before passing to another
/// [`Sink`](crate::Sink).
#[derive(Debug)]
pub struct SinkTo<Chan, C, S, const CH: usize, const N: usize>
where
    Chan: Channel + From<C>,
    C: Channel,
    S: Sink<Chan, CH>,
{
    sink: S,
    _phantom: core::marker::PhantomData<(Chan, C)>,
}

impl<Chan, C, S, const CH: usize, const N: usize> SinkTo<Chan, C, S, CH, N>
where
    Chan: Channel + From<C>,
    C: Channel,
    S: Sink<Chan, CH>,
{
    /// Convert an arbitrary `Sink` type to a different format.
    pub fn new(sink: S) -> Self {
        Self {
            sink,
            _phantom: core::marker::PhantomData,
        }
    }
}

#[allow(single_use_lifetimes)]
impl<Chan, C, S, const CH: usize, const N: usize> Sink<C, N>
    for &mut SinkTo<Chan, C, S, CH, N>
where
    Chan: Channel + From<C>,
    C: Channel,
    S: Sink<Chan, CH>,
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
    fn sink_with(&mut self, iter: &mut dyn Iterator<Item = Frame<C, N>>) {
        self.sink.sink_with(&mut iter.map(Frame::to))
    }
}

#[allow(single_use_lifetimes)]
impl<Chan, C, S, const CH: usize, const N: usize> Sink<C, N>
    for SinkTo<Chan, C, S, CH, N>
where
    Chan: Channel + From<C>,
    C: Channel,
    S: Sink<Chan, CH>,
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
    fn sink_with(&mut self, iter: &mut dyn Iterator<Item = Frame<C, N>>) {
        self.sink.sink_with(&mut iter.map(Frame::to))
    }
}
