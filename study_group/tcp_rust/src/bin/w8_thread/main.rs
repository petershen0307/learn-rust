#[macro_use]
extern crate rocket;

#[post("/api/v1/filehasher", data = "<path>")]
fn file_sha(path: String) -> String {
    let path = std::fs::canonicalize(path).unwrap();
    tcp_listener::file_sha::thread::list_files_sha512(path, 3).join("\n")
}

#[launch]
fn rocket() -> _ {
    let config = rocket::Config {
        port: 8080,
        ..Default::default()
    };
    rocket::custom(config).mount("/", routes![file_sha])
}
