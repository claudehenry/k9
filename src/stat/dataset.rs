use std::fmt::Display;

use super::{mean, trend::Trend, Stat};

pub struct Dataset<T> {
    name: String,
    samples: Vec<T>,
    mean: T,
}

impl<T> From<Trend<'_, T>> for Dataset<T>
where
    T: Stat,
{
    fn from(trend: Trend<T>) -> Self {
        let name = trend.name;
        let mut samples = trend.samples.into_inner().unwrap();
        samples.sort_by(|a, b| {
            a.partial_cmp(b)
                .expect("elements in trend buffer should be comparable")
        });
        let mean = mean(&samples);
        Dataset {
            name,
            samples,
            mean,
        }
    }
}

impl<T> Display for Dataset<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl<T> Dataset<T>
where
    T: Stat,
{
    #[inline]
    pub fn min(&self) -> T {
        *self.samples.first().unwrap_or(&T::default())
    }

    #[inline]
    pub fn max(&self) -> T {
        *self.samples.last().unwrap_or(&T::default())
    }

    #[inline]
    pub fn mean(&self) -> T {
        self.mean
    }

    #[inline]
    pub fn median(&self) -> T {
        let midpoint = (self.samples.len() as f64 / 2.) as usize;
        match self.samples.get(midpoint) {
            Some(t) => *t,
            None => T::default(),
        }
    }

    #[inline]
    pub fn percentile(&self, p: f64) -> T {
        let index = p * (self.samples.len() as f64 - 1.);
        let p0 = index.floor() as usize;
        let p1 = index.ceil() as usize;
        let factor = index - p0 as f64;

        self.samples
            .get(p0)
            .unwrap_or(&T::default())
            .lerp(*self.samples.get(p1).unwrap_or(&T::default()), factor)
    }
}
