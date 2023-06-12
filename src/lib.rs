mod stat;

// pub use stat::trend::{Trend, TrendStat};
pub use stat::trend::Trend;

mod http;

pub use crate::http::{Http, HttpRequest};

mod error;

pub use error::{Error, Result};
