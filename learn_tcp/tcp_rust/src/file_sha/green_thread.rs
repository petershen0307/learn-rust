use tokio::sync::mpsc;
use walkdir::WalkDir;

use crate::file_sha::file_sha512;
use crate::file_sha::{Job, JobResult};

pub async fn list_files_with_workers(path: std::path::PathBuf, workers: usize) -> Vec<String> {
    let (path_tx, path_rx) = async_channel::bounded(workers);
    let (result_tx, mut result_rx) = mpsc::channel(workers);

    // the loop iteration times will not equal to workers number, because we want to put the outer receiver {path_rx} and sender {result_tx} ownership to the worker function
    for _ in 0..workers - 1 {
        let input = path_rx.clone();
        let output = result_tx.clone();
        tokio::spawn(async move { worker(input, output).await });
    }

    // move the outer receiver {path_rx} and sender {result_tx} ownership to the worker function
    // when all senders are out of scope, the channel will be close
    // the key point is all senders should be recycle, otherwise the channel wouldn't close and the spawn task also not stop
    // and the outcome will be program hang
    tokio::spawn(async move { worker(path_rx, result_tx).await });

    tokio::spawn(async move {
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_dir() {
                // skip dir
                continue;
            }
            let r = entry.path().to_path_buf();
            let _ = path_tx.send(Job::Data(r)).await;
        }
    });

    let mut result = Vec::<String>::new();

    while let Some(r) = result_rx.recv().await {
        if let JobResult::Data(v) = r {
            result.push(v);
        }
    }

    result.sort();
    result
}

async fn worker(input: async_channel::Receiver<Job>, output: mpsc::Sender<JobResult>) {
    while let Ok(job) = input.recv().await {
        if let Job::Data(path) = job {
            let mut file = std::fs::File::open(&path).unwrap();
            let sha = file_sha512(&mut file);
            let _ = output
                .send(JobResult::Data(format!("{} {}", path.display(), sha)))
                .await;
        }
    }
}

#[tokio::test]
async fn test_list() {
    let r = list_files_with_workers(std::path::Path::new("./src/bin").to_path_buf(), 3).await;
    println!("{}", r.join("\n"));
}
