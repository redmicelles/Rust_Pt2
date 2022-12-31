// #![deny(clippy::all)]

use futures::executor::block_on;
use tokio::time::{sleep, Duration};
use futures::Future;

async fn get_name() -> String {
    "John".to_string()
}

fn call_api_one() -> impl Future<Output = String> {
    async {
        sleep(Duration::from_secs(10)).await;
        "Api Call One".to_string()
    }
}

async fn call_api_two() -> String {
    sleep(Duration::from_secs(5)).await;
    "Api Call Two".to_string()
}

#[tokio::main]
async fn main() {
    //await using futures
    let name = block_on(get_name());
    println!("{name}");

    //await using tokio
    let one = call_api_one().await;
    println!("{one}");

    let two = call_api_two().await;
    println!("{two}");
}
