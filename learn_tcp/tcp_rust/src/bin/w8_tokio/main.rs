#[macro_use]
extern crate rocket;

#[post("/api/v1/filehasher", data = "<path>")]
async fn file_sha(path: String) -> String {
    let path = std::fs::canonicalize(path).unwrap();
    let o = tcp_listener::file_sha::tokio::list_files_sha512(path).await;
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
