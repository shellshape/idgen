mod generators;

use std::{io, ops::Deref};

use clap::{Parser, Subcommand};
use generators::Generator;
use string_join::Join;

#[derive(Subcommand, Clone)]
enum Commands {
    /// Generate XIDs (see https://github.com/rs/xid)
    Xid(generators::xid::Xid),
    /// Generate UUIDs (aka. GUIDs)
    Uuid(generators::uuid::Uuid),
    /// Generate Snowflake
    Snowflake(generators::snowflake::Snowflake),
}

impl Deref for Commands {
    type Target = dyn Generator<Params = Cli>;

    fn deref(&self) -> &Self::Target {
        match self {
            Commands::Xid(xid) => xid,
            Commands::Uuid(uuid) => uuid,
            Commands::Snowflake(snowflake) => snowflake,
        }
    }
}

#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Set to larger than 1 to generate a list of UIDs.
    #[arg(short, long, default_value_t = 1)]
    number: u16,
}

fn main() {
    let cli = Cli::parse();

    let mut uids = Vec::new();

    for _ in 0..cli.number {
        let res = cli.command.generate(cli.clone());
        if let Err(err) = res {
            eprintln!("Failed generating UID: {}", err);
            return;
        }
        uids.push(res.unwrap());
    }

    if let Err(err) = "\n".write_join(&mut io::stdout(), uids) {
        eprintln!("Failed printing results to STDOUT: {}", err);
    }
}
