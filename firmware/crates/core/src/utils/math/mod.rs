mod percentage;

use std::ops::{RangeInclusive, Sub, Mul, Add, Div};

pub use percentage::*;

/// Map a value from a range to another.
pub fn map<T>(value: T, from: RangeInclusive<T>, to: RangeInclusive<T>) -> T 
    where T: Clone + Copy + Sub<T, Output = T> + Mul<T, Output = T> + Add<T, Output = T> + Div<T, Output = T>
{
    (value - *from.start()) * (*to.end() - *to.start()) / (*from.end() - *from.start()) + *to.start()
}