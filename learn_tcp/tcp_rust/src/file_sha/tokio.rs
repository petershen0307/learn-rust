use std::path::PathBuf;
use std::sync::Arc;

use tokio::sync::Mutex;
use walkdir::WalkDir;

pub async fn list_files_sha512(path: std::path::PathBuf) -> Vec<String> {
    let result = Arc::new(Mutex::new(Vec::new()));

    let mut spawn_join = Vec::new();

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_dir() {
            // skip dir
            continue;
        }
        let r = entry.path().to_path_buf();
        let result = result.clone();
        spawn_join.push(tokio::spawn(async move { worker(r, result).await }));
    }

    // wait all spawn finish
    for spawn in spawn_join {
        tokio::join!(spawn).0.unwrap();
    }

    let mut result = result.lock().await.to_vec();
    result.sort();
    result
}

async fn worker(path: PathBuf, output: Arc<Mutex<Vec<String>>>) {
    let mut file = std::fs::File::open(path.clone()).unwrap();
    let sha = crate::file_sha::file_sha512(&mut file);
    let r = format!("{} {}", path.display(), sha);
    let mut output = output.lock().await;
    output.push(r);
}
