use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, PartialEq, Debug)]
enum Commands {
    /// SET key value [NX | XX] [GET] [EX seconds | PX milliseconds | EXAT unix-time-seconds | PXAT unix-time-milliseconds | KEEPTTL]
    /// https://redis.io/commands/set/
    Set(SetArgs),
}

#[derive(Args, PartialEq, Debug)]
struct SetArgs {
    key: String,
    value: String,
    #[arg(value_enum)]
    nxxx: Option<Nxxx>,
    #[arg(value_enum)]
    get: Option<Get>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Nxxx {
    Nx,
    Xx,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Get {
    Get,
}

#[test]
fn test_parse() {
    // arrange
    let command_line = vec!["ignore", "set", "PP", "v"];
    // act
    let cli = Cli::parse_from(command_line);
    // assert
    let args = SetArgs {
        key: "PP".to_string(),
        value: "v".to_string(),
        nxxx: None,
        get: None,
    };
    assert_eq!(Commands::Set(args), cli.command);
}

#[test]
fn test_parse2() {
    // arrange
    let command_line = vec!["ignore", "set", "PP", "v", "nx"];
    // act
    let cli = Cli::parse_from(command_line);
    // assert
    let args = SetArgs {
        key: "PP".to_string(),
        value: "v".to_string(),
        nxxx: Some(Nxxx::Nx),
        get: None,
    };
    assert_eq!(Commands::Set(args), cli.command);
}
#[test]
fn test_parse3() {
    // arrange
    let command_line = vec!["ignore", "set", "PP", "v", "xx", "get"];
    // act
    let cli = Cli::parse_from(command_line);
    // assert
    let args = SetArgs {
        key: "PP".to_string(),
        value: "v".to_string(),
        nxxx: Some(Nxxx::Xx),
        get: Some(Get::Get),
    };
    assert_eq!(Commands::Set(args), cli.command);
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Set(args) => {
            println!("set key {:?}", args.key);
            println!("set value {:?}", args.value);
        }
    }
}
