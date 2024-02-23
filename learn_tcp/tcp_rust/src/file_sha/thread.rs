use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

use crate::file_sha::file_sha512;
use crate::file_sha::Job;
use crate::file_sha::Result;

use walkdir::WalkDir;

pub fn list_files_sha512(path: std::path::PathBuf, workers: usize) -> Vec<String> {
    let mut result = Vec::new();
    let (path_tx, path_rx) = mpsc::channel();
    let (result_tx, result_rx) = mpsc::channel();

    let path_rx = Arc::new(Mutex::new(path_rx));

    // sha worker
    for _ in 0..workers {
        let path_rx = path_rx.clone();
        let result_tx = result_tx.clone();
        thread::spawn(|| {
            cal_file_sha_worker(path_rx, result_tx);
        });
    }

    // walkDir worker
    thread::spawn(move || {
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_dir() {
                // skip dir
                continue;
            }
            let _ = path_tx.send(Job::Data(entry.path().to_path_buf()));
        }
        // all files are found, go to stop worker
        for _ in 0..workers {
            let _ = path_tx.send(Job::Stop);
        }
    });

    let mut stop_workers = 0_usize;
    loop {
        match result_rx.recv().unwrap() {
            Result::Data(r) => {
                result.push(r);
            }
            Result::Stop => {
                stop_workers += 1;
                if stop_workers == workers {
                    break;
                }
            }
        }
    }
    result.sort();
    result
}

fn cal_file_sha_worker(input: Arc<Mutex<mpsc::Receiver<Job>>>, output: mpsc::Sender<Result>) {
    loop {
        let job: Job;
        {
            let input = input.lock().unwrap();
            job = input.recv().unwrap();
        }
        match job {
            Job::Data(path) => {
                let mut file = std::fs::File::open(&path).unwrap();
                let sha = file_sha512(&mut file);
                let _ = output.send(Result::Data(format!("{} {}", path.display(), sha)));
            }
            Job::Stop => {
                let _ = output.send(Result::Stop);
                break;
            }
        }
    }
}
