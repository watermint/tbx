use std::ops::{Add, Div, Mul, Rem, Sub};

pub trait Number: NumberOps+Sized {
}

/// Basic numeric operations.
pub trait NumberOps<Rhs = Self, Output = Self>:
Add<Rhs, Output=Output>
+ Sub<Rhs, Output=Output>
+ Mul<Rhs, Output=Output>
+ Div<Rhs, Output=Output>
+ Rem<Rhs, Output=Output> {}

impl<T, Rhs, Output> NumberOps<Rhs, Output> for T where T: Add<Rhs, Output=Output>
+ Sub<Rhs, Output=Output>
+ Mul<Rhs, Output=Output>
+ Div<Rhs, Output=Output>
+ Rem<Rhs, Output=Output> {}
