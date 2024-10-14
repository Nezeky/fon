use crate::chan::{Ch16, Ch24, Ch32, Ch64};

pub trait Sealed {}
impl Sealed for Ch16 {}
impl Sealed for Ch24 {}
impl Sealed for Ch32 {}
impl Sealed for Ch64 {}
