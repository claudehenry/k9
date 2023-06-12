use std::{sync::Arc, time::Duration};

use k9::Http;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    const AGENTS: usize = 4;
    let http = Http::arcd();

    let agents: Vec<_> = (0..AGENTS)
        .map(|agent_id| {
            log::info!("Spanwed agent[{}]", agent_id);

            let http = http.clone();
            tokio::spawn(async move {
                agent(http).await;

                log::info!("Terminated agent[{}]", agent_id);
            })
        })
        .collect();

    futures::future::join_all(agents).await;

    log::info!("Finished test run!");

    let http = Arc::try_unwrap(http)
        .map_err(|_| "failed to get unique ownership of the Http client, is it still in use?")
        .unwrap();

    let request_duration = http.request_duration.dataset();
    println!(
        "{} average: {:.1?}",
        request_duration,
        request_duration.mean()
    );
}

/// Represents an Agent's lifecycle.
async fn agent(http: Arc<Http<'_>>) {
    const URL: &str = "https://example.com/";

    for i in 0..10 {
        log::info!("Request #{}", i);
        let response = match http.get(URL).send().await {
            Ok(r) => match r.text().await {
                Ok(t) => t,
                Err(err) => {
                    log::error!("{}", err);
                    continue;
                }
            },
            Err(err) => {
                log::error!("{}", err);
                continue;
            }
        };

        log::trace!("Response {}", response);

        let wait = Duration::from_millis(100 + rand::random::<u64>() % 1000);
        log::info!("Waiting {:.1?}", wait);
        tokio::time::sleep(wait).await;
    }
}
