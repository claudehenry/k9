use std::time::Duration;

use k9::{Http, HttpRequest, Result, Trend};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    const URL: &str = "https://example.com/";

    let http = Http::new();
    let query_requests_duration = Trend::new("request-with-query-duration");

    for i in 0..10 {
        log::info!("Round {} with query", i);

        if let Err(err) = send_request(
            http.get(URL)
                .query(&[("test", "value"), ("another", "1")])
                .trend(&query_requests_duration),
        )
        .await
        {
            log::error!("{}", err);
            continue;
        }

        log::info!("Round {} no query", i);

        if let Err(err) = send_request(http.get(URL)).await {
            log::error!("{}", err);
            continue;
        }

        let wait = Duration::from_millis(100 + rand::random::<u64>() % 1000);
        log::info!("Waiting {:.1?}", wait);
        tokio::time::sleep(wait).await;
    }

    log::info!("Finished test run!");

    print_trend(http.request_duration);
    print_trend(query_requests_duration);
}

async fn send_request(request: HttpRequest<'_>) -> Result<()> {
    let response = request.send().await?.text().await?;
    log::trace!("Response {}", response);
    Ok(())
}

fn print_trend(trend: Trend<'_, Duration>) {
    let data = trend.dataset();
    println!(
        // Don't take this as formatting advice, it _just about_ happens to align the names of the
        // trends hard coded into this example. Stay tuned for more ergonomic reporting.
        "{}    \taverage: {:.1?}    median: {:.1?}    p(90): {:.1?}    p(99): {:.1?}",
        data,
        data.mean(),
        data.median(),
        data.percentile(0.90),
        data.percentile(0.99),
    );
}
