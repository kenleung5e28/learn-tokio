use tokio::select;
use tokio::task::JoinSet;
use tokio::time::{sleep, Duration};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    let cancel_token = CancellationToken::new();
    let mut set = JoinSet::new();
    for i in 0..10 {
        let cancel_token_clone = cancel_token.clone();
        set.spawn(async move {
            select! {
                _ = sleep(Duration::from_secs(10 - i)) => {
                    println!("Thread {} awakes", i);
                }
                _ = cancel_token_clone.cancelled() => {
                    println!("Thread {} receives cancellation signal", i)
                }
            }
            i
        });
    }
    sleep(Duration::from_secs(3)).await;
    cancel_token.cancel();
    while let Some(res) = set.join_next().await {
        match res {
            Ok(i) => println!("Thread {} completes", i),
            Err(e) => println!("Error occurs: {}", e.to_string()),
        }
    }
}
