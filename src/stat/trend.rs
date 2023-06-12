use std::sync::{Arc, Mutex};

use crate::Result;

use super::{dataset::Dataset, Stat};

pub struct Trend<'a, T> {
    // todo: perhaps this doesn't have to be public in the module if I move the dataset back in
    // here... though this module will likely grow.
    pub(super) name: String,
    // todo: there might be more efficient things to do, but I have not run up against this as a
    // bottleneck yet.
    pub(super) samples: Mutex<Vec<T>>,

    pub(super) also: Vec<&'a Trend<'a, T>>,
}

impl<'a, T> Trend<'a, T> {
    pub fn also(mut self, trend: &'a Trend<'a, T>) -> Self {
        self.also.push(trend);
        self
    }
}

impl<T> Trend<'_, T>
where
    T: Stat,
{
    pub fn dataset(self) -> Dataset<T> {
        self.into()
    }
}

impl<T> Trend<'_, T>
where
    T: Copy,
{
    pub fn new<S: AsRef<str>>(name: S) -> Self {
        Self {
            name: name.as_ref().to_string(),
            samples: Mutex::new(vec![]),
            also: vec![],
        }
    }

    pub fn arc<S: AsRef<str>>(name: S) -> Arc<Self> {
        Arc::new(Self::new(name))
    }

    pub async fn add(&self, datum: T) -> Result<()> {
        for &trend in &self.also {
            // Push is used here instead of
            //     trend.add(datum).await;
            // for 2 reasons:
            // 1. If we were to use trend.add(), and we had created a cycle:
            //     a = Trend
            //     b = Trend
            //     a.also(b)
            //     b.also(a)
            //   then we would recurse infinitely when calling a.add(..)
            // 2. The compiler does not support recursive async fn
            trend.samples.lock()?.push(datum);
        }
        self.samples.lock()?.push(datum);
        Ok(())
    }
}
