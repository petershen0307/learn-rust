use std::{thread::sleep, time::Duration};

use crossbeam_channel::SendError;

#[tokio::main()]
async fn main() {
    // let sub = tracing_subscriber::fmt().compact().with_file(true).with_line_number(true).with_thread_ids(true).with_target(false).finish();
    tracing_subscriber::fmt()
        // enable everything
        .with_max_level(tracing::Level::TRACE)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        // sets this to be the default, global collector for this application.
        .init();

    let (r, s) = crossbeam_channel::unbounded();

    let handler = tokio::spawn(async move {
        let span = tracing::span!(tracing::Level::INFO, "thread1");
        let _enter = span.enter();
        sleep(Duration::from_secs(5));
        match r.send("hello, kiwi".to_owned()) {
            Ok(()) => {}
            Err(SendError(err)) => {
                println!("{:?}", err);
            }
        }
    });

    let handler2 = tokio::spawn(async move {
        let span = tracing::span!(tracing::Level::INFO, "thread2");
        let _enter = span.enter();
        while let Ok(result) = s.recv() {
            println!("{}", result);
        }
        println!("t2 exit");
    });

    sleep(Duration::from_millis(500));

    // let _ = tokio::join!(handler2, handler);
}
