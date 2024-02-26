use std::collections::VecDeque;
use std::sync::Arc;

use tokio::sync::Mutex;
use walkdir::WalkDir;

use crate::file_sha::{Job, JobResult};

pub async fn list_files_with_workers(path: std::path::PathBuf, workers: u8) -> Vec<String> {
    let path_queue = Arc::new(Mutex::new(VecDeque::new()));
    let result_queue = Arc::new(Mutex::new(VecDeque::new()));
    let mut v = Vec::new();
    for _ in 0..workers {
        let input = path_queue.clone();
        let output = result_queue.clone();
        v.push(tokio::spawn(async move { worker(input, output).await }));
    }

    v.push(tokio::spawn(async move {
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_dir() {
                // skip dir
                continue;
            }
            let r = entry.path().to_path_buf();
            path_queue.lock().await.push_back(Job::Data(r));
        }
        for _ in 0..workers {
            path_queue.lock().await.push_back(Job::Stop);
        }
    }));

    let mut result = Vec::<String>::new();
    let mut i = 0;
    loop {
        match result_queue.lock().await.pop_front() {
            None => tokio::time::sleep(tokio::time::Duration::from_millis(1)).await,
            Some(v) => match v {
                JobResult::Stop => {
                    i += 1;
                    if i == workers {
                        break;
                    }
                }
                JobResult::Data(r) => result.push(r),
            },
        }
    }

    for i in v {
        tokio::join!(i).0.unwrap();
    }

    result.sort();
    result
}

async fn worker(input: Arc<Mutex<VecDeque<Job>>>, output: Arc<Mutex<VecDeque<JobResult>>>) {
    loop {
        let path: Job;
        {
            let mut receiver = input.lock().await;
            path = match receiver.pop_front() {
                None => {
                    tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
                    continue;
                }
                Some(v) => v,
            }
        }
        match path {
            Job::Data(path) => {
                //tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                let mut file = std::fs::File::open(path.clone()).unwrap();
                let sha = crate::file_sha::file_sha512(&mut file);
                let r = format!("{} {}", path.display(), sha);
                // println!("{}", r);
                output.lock().await.push_back(JobResult::Data(r));
            }
            Job::Stop => {
                output.lock().await.push_back(JobResult::Stop);
                break;
            }
        }
    }
}

#[tokio::test]
async fn test_list() {
    let r = list_files_with_workers(std::path::Path::new("./src/bin").to_path_buf(), 3).await;
    println!("{}", r.join("\n"));
}
