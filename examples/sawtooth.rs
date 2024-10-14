use fon::{
    pos::Mono,
    samp::{Samp16, Samp32},
    Audio,
};

fn main() {
    // Create mono 32-bit floating point audio buffer.
    let mut a = Audio::<Samp32, 1>::with_silence(48_000, 256);
    let mut counter = 0.0;
    for f in a.iter_mut() {
        f[Mono] = counter.into();
        counter += 0.05;
        counter %= 1.0;
    }

    // Convert to 16-Bit audio format
    let mut audio = Audio::<Samp16, 1>::with_audio(48_000, &a);

    // Print out converted wave.
    for (sample, other) in
        audio.as_i16_slice().iter().zip(a.as_f32_slice().iter())
    {
        println!("{} {}", sample, other);
    }
}
