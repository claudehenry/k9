use std::{fmt::Display, ops::Add, time::Duration};

pub mod dataset;
pub mod trend;

// basically an explicit way to control what types can be trended. if you can do the following
// math with it, then it can Stat, thus you can make a trend with it.
pub trait Stat: PartialOrd + Default + Copy + Add<Output = Self> {
    fn div_count(&self, count: usize) -> Self;

    // todo: unchecked factor is normalized
    /// Lineraly interpolate from `self` to `towards`, advancing by factor
    fn lerp(&self, towards: Self, factor: f64) -> Self;

    type DisplayAs: Display;
    fn repr(&self) -> Self::DisplayAs;
}

impl Stat for f64 {
    fn div_count(&self, count: usize) -> Self {
        *self / count as f64
    }

    fn lerp(&self, towards: Self, factor: f64) -> Self {
        self + (towards - self) * factor
    }

    type DisplayAs = Self;
    fn repr(&self) -> Self::DisplayAs {
        *self
    }
}

impl Stat for Duration {
    fn div_count(&self, count: usize) -> Self {
        self.checked_div(count as u32).unwrap_or(Duration::ZERO)
    }

    fn lerp(&self, towards: Self, factor: f64) -> Self {
        self.checked_add(
            towards
                .checked_sub(*self)
                .unwrap_or(Duration::ZERO)
                .mul_f64(factor),
        )
        .unwrap_or(Duration::ZERO)
    }

    type DisplayAs = String;
    fn repr(&self) -> Self::DisplayAs {
        // todo: too precise & long
        format!("{:.1?}", self)
    }
}

fn mean<T>(samples: &Vec<T>) -> T
where
    T: Stat,
{
    samples
        .iter()
        .fold(T::default(), |acc, sample| acc + *sample)
        .div_count(samples.len())
}
