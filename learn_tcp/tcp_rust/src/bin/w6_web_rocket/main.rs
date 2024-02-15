#[macro_use]
extern crate rocket;

#[post("/v1/filehasher", data = "<path>")]
fn file_sha(path: String) -> String {
    tcp_listener::file_sha::list_files_sha512(std::path::Path::new(&path)).join("\n")
}

#[launch]
fn rocket() -> _ {
    let config = rocket::Config {
        port: 8080,
        ..Default::default()
    };
    rocket::custom(config).mount("/", routes![file_sha])
}
