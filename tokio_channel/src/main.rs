use tokio::sync::watch;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = watch::channel("hello");

    tokio::spawn(async move {
        // Use the equivalent of a "do-while" loop so the initial value is
        // processed before awaiting the `changed()` future.
        loop {
            sleep(Duration::from_millis(200)).await;
            println!("{}! ", *rx.borrow_and_update());
            if rx.changed().await.is_err() {
                println!("leave");
                break;
            }
        }
    });

    sleep(Duration::from_millis(100)).await;
    tx.send("world").unwrap();
    drop(tx);
    sleep(Duration::from_millis(200)).await;
}
/*
drop channel before read then the green thread will never read the data
*/
