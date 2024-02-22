use std::time;

use tokio::sync::mpsc;

#[tokio::test]
async fn test_mpsc_channel() {
    mpsc_channel_example().await;
}

async fn mpsc_channel_example() {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    tokio::spawn(async move {
        tx.send("sending from first handle").await;
    });

    tokio::spawn(async move {
        std::thread::sleep(time::Duration::from_millis(100));
        tx2.send("sending from second handle").await;
    });

    while let Some(message) = rx.recv().await {
        println!("GOT = {}", message);
    }
    println!("leave")
}

#[tokio::test]
async fn test_pipeline() {
    let (producer_tx, mut transformer_rx) = mpsc::channel(32);

    let (transformer_tx, mut consumer_rx) = mpsc::channel(32);

    let producer_handle = tokio::spawn(async move {
        for i in 1..=10 {
            println!("Producing {}", i);

            producer_tx.send(i).await.unwrap();
        }
    });

    let transformer_handle = tokio::spawn(async move {
        while let Some(i) = transformer_rx.recv().await {
            println!("Transforming {}", i);

            transformer_tx.send(i * i).await.unwrap();
        }
    });

    let consumer_handle = tokio::spawn(async move {
        while let Some(i) = consumer_rx.recv().await {
            println!("Consumed {}", i);
        }
    });

    tokio::try_join!(producer_handle, transformer_handle, consumer_handle).unwrap();
}
