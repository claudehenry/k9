use std::time::Duration;

use k9::Http;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    const URL: &str = "https://example.com/";
    let http = Http::new();

    for i in 0..10 {
        log::info!("Request #{}", i);
        let response = match http.get(URL).send().await {
            //                     _____________________________
            //                    | And preeeeetty terrible...  \
            //                    \__  _ contine... twice. smh  /
            //                       V
            // this is a very verbose way to handle errors, and I do aim to have a runtime wrapper,
            // which would favor idiomatic ? error handling. Left as an exercise to the reader!
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

    log::info!("Finished test run!");

    let request_duration = http.request_duration.dataset();
    println!(
        "{} average: {:.1?}",
        request_duration,
        request_duration.mean()
    );
}
