#[macro_use]
extern crate rocket;

#[post("/api/v1/filehasher", data = "<path>")]
async fn file_sha(path: String) -> String {
    let path = std::fs::canonicalize(path).unwrap();
    let o = tcp_listener::file_sha::green_thread::list_files_with_workers(path, 8).await;
    o.join("\n")
}

#[launch]
async fn rocket() -> _ {
    let config = rocket::Config {
        port: 8080,
        ..Default::default()
    };
    rocket::custom(config).mount("/", routes![file_sha])
}
