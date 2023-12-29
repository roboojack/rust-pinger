use reqwest::Client;
use std::time::Instant;
use std::env;

async fn perform_http_get(url: &str) -> Result<(u64, u128), reqwest::Error> {
    let client = Client::new();
    let start_time = Instant::now();
    let response = client.get(url).send().await?;
    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time).as_millis();
    let content_length = response.content_length().unwrap_or(0);

    Ok((content_length, elapsed_time))
}

async fn perform_http_get_multiple_times(url: &str, num_times: u32) -> Result<(), reqwest::Error> {
    let mut total_bytes = 0;
    let mut total_latency = 0;
    let mut min_latency = u128::MAX;
    let mut max_latency = 0;


    println!("Pinging {} {} times", url, num_times);
    for _ in 0..num_times {
        let (bytes, latency) = perform_http_get(url).await?;
        total_bytes += bytes;
        total_latency += latency;
        min_latency = min_latency.min(latency);
        max_latency = max_latency.max(latency);
    }

    let mean_latency = total_latency / num_times as u128;

    println!("Total bytes: {}", total_bytes);
    println!("Minimum latency: {} ms", min_latency);
    println!("Maximum latency: {} ms", max_latency);
    println!("Mean latency: {} ms", mean_latency);

    Ok(())
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: cargo run <url> <num_times>");
        return;
    }

    let url = &args[1];
    let num_times: u32 = match args[2].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid number of times");
            return;
        }
    };

    if let Err(err) = perform_http_get_multiple_times(url, num_times).await {
        eprintln!("Error: {}", err);
    }
}
