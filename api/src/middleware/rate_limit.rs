use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use axum::extract::{ConnectInfo, Request, State};
use axum::http::{header::RETRY_AFTER, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

use crate::models::ApiErrorResponse;

const WINDOW: Duration = Duration::from_secs(60);
const PRUNE_AT: usize = 10_000;

/// Fixed-window counter per client address, kept in this process only.
///
/// The address is the TCP peer. Behind a reverse proxy every client shares one,
/// and forwarded headers are not trusted, so limit at the proxy in that setup.
#[derive(Clone)]
pub struct RateLimiter {
    max: u32,
    hits: Arc<Mutex<HashMap<IpAddr, (Instant, u32)>>>,
}

impl RateLimiter {
    /// `max` requests per minute; 0 disables the limit.
    pub fn new(max: u32) -> Self {
        Self {
            max,
            hits: Arc::default(),
        }
    }

    /// On refusal returns how long until the window reopens.
    fn check(&self, client: IpAddr, now: Instant) -> Result<(), Duration> {
        if self.max == 0 {
            return Ok(());
        }
        let mut hits = self.hits.lock().unwrap_or_else(|e| e.into_inner());
        if hits.len() >= PRUNE_AT {
            hits.retain(|_, (start, _)| now.duration_since(*start) < WINDOW);
        }

        let (start, count) = hits.entry(client).or_insert((now, 0));
        if now.duration_since(*start) >= WINDOW {
            (*start, *count) = (now, 0);
        }
        if *count >= self.max {
            return Err(WINDOW - now.duration_since(*start));
        }
        *count += 1;
        Ok(())
    }
}

pub async fn limit(State(limiter): State<RateLimiter>, request: Request, next: Next) -> Response {
    let client = request
        .extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED), |info| info.0.ip());

    match limiter.check(client, Instant::now()) {
        Ok(()) => next.run(request).await,
        Err(wait) => {
            tracing::warn!(%client, "rate limit reached");
            (
                StatusCode::TOO_MANY_REQUESTS,
                [(RETRY_AFTER, wait.as_secs().max(1).to_string())],
                axum::Json(ApiErrorResponse {
                    message: "Too many requests".to_string(),
                }),
            )
                .into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_each_client_and_reopens_after_the_window() {
        let limiter = RateLimiter::new(2);
        let (a, b) = (IpAddr::from([10, 0, 0, 1]), IpAddr::from([10, 0, 0, 2]));
        let start = Instant::now();

        assert!(limiter.check(a, start).is_ok());
        assert!(limiter.check(a, start).is_ok());
        assert_eq!(
            limiter.check(a, start + Duration::from_secs(20)),
            Err(Duration::from_secs(40))
        );
        assert!(limiter.check(b, start).is_ok());
        assert!(limiter.check(a, start + WINDOW).is_ok());
    }

    #[test]
    fn zero_disables_the_limit() {
        let limiter = RateLimiter::new(0);
        let client = IpAddr::from([10, 0, 0, 1]);
        for _ in 0..100 {
            assert!(limiter.check(client, Instant::now()).is_ok());
        }
    }
}
