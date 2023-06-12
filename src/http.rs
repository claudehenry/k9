use std::{
    ops::Deref,
    sync::Arc,
    time::{Duration, Instant},
};

use {
    lazy_static::lazy_static,
    reqwest::{
        header::{HeaderName, HeaderValue},
        Client, ClientBuilder, IntoUrl, RequestBuilder,
    },
    serde::Serialize,
};

use crate::{Result, Trend};

pub struct Http<'a> {
    pub request_duration: Trend<'a, Duration>,
}

lazy_static! {
    // Safety: a client with default
    static ref CLIENT: Client = ClientBuilder::new().build().expect("failed to initialize reqwest Client with default settings");
}

// todo: clarify what the docstrings copied from reqwest mean by '# Errors', presumably it's a
// panic. prehaps there is a way to convert them to fallibles as I wish for this not to panic
impl Http<'_> {
    /// Create a new http client.
    pub fn new() -> Self {
        Http {
            request_duration: Trend::new("http-request-duration"),
        }
    }

    /// Convenience method to make a new client wrapped in an [Arc](std::sync::Arc).
    pub fn arcd() -> Arc<Self> {
        Arc::new(Self::new())
    }

    /// Convenience method to make a `GET` request to a URL.
    ///
    /// # Errors
    ///
    /// This method fails whenever the supplied `Url` cannot be parsed.
    pub fn get<U: IntoUrl>(&self, url: U) -> HttpRequest {
        HttpRequest {
            http: self,
            // builder: CLIENT.with(|c| c.get(url)),
            builder: CLIENT.get(url),
            trend: None,
        }
    }

    /// Convenience method to make a `POST` request to a URL.
    ///
    /// # Errors
    ///
    /// This method fails whenever the supplied `Url` cannot be parsed.
    pub fn post<U: IntoUrl>(&self, url: U) -> HttpRequest {
        HttpRequest {
            http: self,
            // builder: CLIENT.with(|c| c.post(url)),
            builder: CLIENT.post(url),
            trend: None,
        }
    }

    /// Convenience method to make a `PUT` request to a URL.
    ///
    /// # Errors
    ///
    /// This method fails whenever the supplied `Url` cannot be parsed.
    pub fn put<U: IntoUrl>(&self, url: U) -> HttpRequest {
        HttpRequest {
            http: self,
            // builder: CLIENT.with(|c| c.put(url)),
            builder: CLIENT.put(url),
            trend: None,
        }
    }

    /// Convenience method to make a `PATCH` request to a URL.
    ///
    /// # Errors
    ///
    /// This method fails whenever the supplied `Url` cannot be parsed.
    pub fn patch<U: IntoUrl>(&self, url: U) -> HttpRequest {
        HttpRequest {
            http: self,
            // builder: CLIENT.with(|c| c.patch(url)),
            builder: CLIENT.patch(url),
            trend: None,
        }
    }

    /// Convenience method to make a `DELETE` request to a URL.
    ///
    /// # Errors
    ///
    /// This method fails whenever the supplied `Url` cannot be parsed.
    pub fn delete<U: IntoUrl>(&self, url: U) -> HttpRequest {
        HttpRequest {
            http: self,
            // builder: CLIENT.with(|c| c.delete(url)),
            builder: CLIENT.delete(url),
            trend: None,
        }
    }

    /// Convenience method to make a `HEAD` request to a URL.
    ///
    /// # Errors
    ///
    /// This method fails whenever the supplied `Url` cannot be parsed.
    pub fn head<U: IntoUrl>(&self, url: U) -> HttpRequest {
        HttpRequest {
            http: self,
            // builder: CLIENT.with(|c| c.head(url)),
            builder: CLIENT.head(url),
            trend: None,
        }
    }
}

impl Default for Http<'_> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct HttpRequest<'a> {
    http: &'a Http<'a>,
    builder: RequestBuilder,
    trend: Option<&'a Trend<'a, Duration>>,
}

impl<'a> HttpRequest<'a> {
    pub fn trend(mut self, trend: &'a Trend<Duration>) -> Self {
        self.trend = Some(trend);
        self
    }
}

impl HttpRequest<'_> {
    pub fn header<K, V>(self, key: K, value: V) -> Self
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
    {
        let Self {
            http,
            builder,
            trend,
        } = self;
        let builder = builder.header(key, value);

        Self {
            http,
            builder,
            trend,
        }
    }

    pub fn body<T: Into<reqwest::Body>>(self, body: T) -> Self {
        let Self {
            http,
            builder,
            trend,
        } = self;
        let builder = builder.body(body);

        Self {
            http,
            builder,
            trend,
        }
    }

    pub fn json<T: Serialize + ?Sized>(self, json: &T) -> Self {
        let Self {
            http,
            builder,
            trend,
        } = self;
        let builder = builder.json(json);

        Self {
            http,
            builder,
            trend,
        }
    }

    pub fn query<T: Serialize + ?Sized>(self, query: &T) -> Self {
        let Self {
            http,
            builder,
            trend,
        } = self;
        let builder = builder.query(query);

        Self {
            http,
            builder,
            trend,
        }
    }

    // this intentionally shadows the definition for send on RequestBuilder
    pub async fn send(self) -> Result<reqwest::Response> {
        let start = Instant::now();
        let response_result = self.builder.send().await?;

        let elapsed = start.elapsed();

        self.http.request_duration.add(elapsed).await?;
        if let Some(trend) = self.trend {
            trend.add(elapsed).await?;
        }

        Ok(response_result)
    }
}

/// this is implemented to passthru all the functions, but as it turns out, I totally need to
/// rewrite all of them... (because the derefed funcs return owned reqwest::RequestBuilder and
/// I need them to be k9::HttpRequest)
/// todo: :point_up:
impl Deref for HttpRequest<'_> {
    type Target = RequestBuilder;

    fn deref(&self) -> &Self::Target {
        &self.builder
    }
}
