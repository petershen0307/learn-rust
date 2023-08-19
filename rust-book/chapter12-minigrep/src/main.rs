use std::env;
use std::process;

//$env:IGNORE_CASE=1; cargo run -- who .\poem.txt;Remove-Item Env:\IGNORE_CASE
//cargo run -- who .\poem.txt
fn main() {
    let args: Vec<String> = env::args().collect();

    // add the package name chapter12_minigrep to use the lib crate in binary crate
    // also the package name will change hyphens to underscore
    let config = chapter12_minigrep::Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = chapter12_minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
