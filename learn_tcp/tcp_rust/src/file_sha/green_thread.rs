use tokio::sync::mpsc;
use tokio::sync::watch;
use walkdir::WalkDir;

pub async fn list_files_with_workers(path: std::path::PathBuf, workers: u8) -> Vec<String> {
    let mut result = Vec::new();
    let (path_sender, path_receiver) = watch::channel(std::path::PathBuf::new());
    let (result_sender, mut result_receiver) = mpsc::channel(workers as usize);
    let path = path.clone();

    for _ in 0..workers {
        let input = path_receiver.clone();
        let output = result_sender.clone();
        tokio::spawn(async move { worker(input, output).await });
    }
    tokio::spawn(async move {
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_dir() {
                // skip dir
                continue;
            }
            let _ = path_sender.send(entry.path().to_path_buf());
        }
        path_sender.closed().await;
    });

    loop {
        match result_receiver.recv().await {
            None => break,
            Some(r) => result.push(r),
        }
    }

    result
}

async fn worker(mut receiver: watch::Receiver<std::path::PathBuf>, report: mpsc::Sender<String>) {
    while receiver.changed().await.is_ok() {
        let path = receiver.borrow_and_update().clone();
        let mut file = std::fs::File::open(path.clone()).unwrap();
        let sha = crate::file_sha::file_sha512(&mut file);
        let _ = report.send(format!("{} {}", path.display(), sha)).await;
    }
    report.closed().await;
}
