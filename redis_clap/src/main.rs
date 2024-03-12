use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// SET key value [NX | XX] [GET] [EX seconds | PX milliseconds | EXAT unix-time-seconds | PXAT unix-time-milliseconds | KEEPTTL]
    /// https://redis.io/commands/set/
    Set(SetArgs),
}

#[derive(Args)]
struct SetArgs {
    key: String,
    value: String,
    #[arg(long)]
    get: bool,
    // ----
    #[arg(long)]
    nx: bool,
    #[arg(long)]
    xx: bool,
    // ----
    #[arg(long)]
    ex: Option<u64>,
    #[arg(long)]
    px: Option<u64>,
    #[arg(long)]
    exat: Option<u64>,
    #[arg(long)]
    pxat: Option<u64>,
    #[arg(long)]
    keepttl: bool,
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
