use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use walkdir::WalkDir;

pub async fn list_files_with_workers(path: std::path::PathBuf, workers: u8) -> Vec<String> {
    let (path_sender, path_receiver) =
        mpsc::channel::<Option<std::path::PathBuf>>(workers as usize);
    let (result_sender, mut result_receiver) = mpsc::channel::<Option<String>>(workers as usize);
    let arc_path_receiver = Arc::new(Mutex::new(path_receiver));
    let mut v = Vec::new();
    for _ in 0..workers {
        let input = arc_path_receiver.clone();
        let output = result_sender.clone();
        v.push(tokio::spawn(async move { worker(input, output).await }));
    }

    v.push(tokio::spawn(async move {
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_dir() {
                // skip dir
                continue;
            }
            let r = entry.path().to_path_buf();
            //println!("{}", r.display());
            let _ = path_sender.send(Some(r)).await;
        }
        for _ in 0..workers {
            path_sender.send(None).await;
        }
    }));

    let mut result = Vec::<String>::new();
    let mut i = 0;
    loop {
        match result_receiver.recv().await.unwrap() {
            None => {
                i += 1;
                if i == workers {
                    break;
                }
            }
            Some(r) => result.push(r),
        }
    }

    for i in v {
        tokio::join!(i).0.unwrap();
    }

    result.sort();
    result
}

async fn worker(
    input: Arc<Mutex<mpsc::Receiver<Option<std::path::PathBuf>>>>,
    output: mpsc::Sender<Option<String>>,
) {
    loop {
        let path: Option<std::path::PathBuf>;
        {
            let mut receiver = input.lock().await;
            path = receiver.recv().await.unwrap();
        }
        match path {
            Some(path) => {
                //tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                let mut file = std::fs::File::open(path.clone()).unwrap();
                let sha = crate::file_sha::file_sha512(&mut file);
                let r = format!("{} {}", path.display(), sha);
                // println!("{}", r);
                let _ = output.send(Some(r)).await;
            }
            None => {
                output.send(None).await;
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

#[tokio::test]
async fn test_worker2() {
    let (path_sender, path_receiver) = mpsc::channel::<Option<std::path::PathBuf>>(3);
    let (result_sender, mut result_receiver) = mpsc::channel::<Option<String>>(3);
    let arc_path_receiver = Arc::new(Mutex::new(path_receiver));
    let mut v = Vec::new();
    for i in 0..2 {
        let input = arc_path_receiver.clone();
        let output = result_sender.clone();
        v.push(tokio::spawn(async move { worker(input, output).await }));
    }
    path_sender
        .send(Some(
            std::path::Path::new("./src/bin/w7_web_rocket/main.rs").to_path_buf(),
        ))
        .await;
    path_sender
        .send(Some(std::path::Path::new("./src/stdin.rs").to_path_buf()))
        .await;
    path_sender.send(None).await;
    path_sender.send(None).await;

    println!("{}", result_receiver.recv().await.unwrap().unwrap());
    println!("{}", result_receiver.recv().await.unwrap().unwrap());

    for i in v {
        tokio::join!(i).0.unwrap();
    }

    println!("leave main");
}

#[tokio::test]
async fn test_worker() {
    let (path_sender, path_receiver) = mpsc::channel::<Option<std::path::PathBuf>>(3);
    let (result_sender, mut result_receiver) = mpsc::channel(3);
    let arc_path_receiver = Arc::new(Mutex::new(path_receiver));
    let mut v = Vec::new();
    for i in 0..2 {
        let input = arc_path_receiver.clone();
        let output = result_sender.clone();
        v.push(tokio::spawn(async move {
            loop {
                let path: Option<std::path::PathBuf>;
                {
                    let mut receiver = input.lock().await;
                    println!("worker{}", i);
                    path = receiver.recv().await.unwrap();
                }
                match path {
                    Some(path) => {
                        //tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                        let mut file = std::fs::File::open(path.clone()).unwrap();
                        let sha = crate::file_sha::file_sha512(&mut file);
                        println!("worker{} {}", i, sha);
                        let _ = output.send(format!("{} {}", path.display(), sha)).await;
                    }
                    None => {
                        println!("worker leave{}", i);
                        break;
                    }
                }
            }
        }));
    }
    path_sender
        .send(Some(
            std::path::Path::new("./src/bin/w7_web_rocket/main.rs").to_path_buf(),
        ))
        .await;
    path_sender
        .send(Some(std::path::Path::new("./src/stdin.rs").to_path_buf()))
        .await;
    path_sender.send(None).await;
    path_sender.send(None).await;

    println!("{}", result_receiver.recv().await.unwrap());
    println!("{}", result_receiver.recv().await.unwrap());

    for i in v {
        tokio::join!(i).0.unwrap();
    }

    println!("leave main");
}

enum Job {
    Data(i32),
    Stop,
}
#[tokio::test]
async fn test_main() {
    let (tx, rx) = mpsc::channel::<Option<std::path::PathBuf>>(10);
    let rx = Arc::new(Mutex::new(rx));
    let mut v = Vec::new();
    for _ in 0..3 {
        let rx = rx.clone();
        v.push(tokio::spawn(async move {
            loop {
                let mut rx = rx.lock().await;
                let job = rx.recv().await;
                let job = job.unwrap();
                match job {
                    Some(n) => {
                        println!("{}", n.display());
                    }
                    None => {
                        println!("worker leave");
                        return;
                    }
                }
            }
        }));
    }

    for _ in 0..3 {
        let _ = tx
            .send(Some(
                std::path::Path::new("./src/bin/w7_web_rocket/main.rs").to_path_buf(),
            ))
            .await
            .unwrap();
    }

    for _ in 0..3 {
        let _ = tx.send(None).await.unwrap();
    }
    for i in v {
        tokio::join!(i).0.unwrap();
    }
}
