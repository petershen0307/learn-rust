#[macro_use]
extern crate rocket;

#[post("/v1/filehasher", data = "<path>")]
fn file_sha(path: String) -> String {
    format!("input path={}", path)
}

#[launch]
fn rocket() -> _ {
    let config = rocket::Config {
        port: 8080,
        ..Default::default()
    };
    rocket::custom(config).mount("/", routes![file_sha])
}
