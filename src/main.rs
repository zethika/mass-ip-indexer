use clap::{Parser, Subcommand};
use logger::DummyLogger;
use std::collections::VecDeque;
use std::str::FromStr;
use crate::ip::ip_range::IpRange;
use crate::ip::ip_range_bounds::IpRangeBounds;

mod ip;
mod logger;

#[derive(Parser, Debug)]
#[clap(author = "Zethika", version, about)]
/// A tool to index all active ips in a given range
struct Arguments {
    #[clap(short, long, parse(from_occurrences))]
    verbosity: usize,
    #[clap(subcommand)]
    cmd: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    /// Index ips in the given range(s).
    Index {
        #[clap(long, default_value_t = String::from("0-255"), forbid_empty_values = true, validator = ip::ip_range_bounds::IpRangeBounds::validate_ip_range)]
        /// Range 0
        r0: String,

        #[clap(long, default_value_t = String::from("0-255"), forbid_empty_values = true, validator = ip::ip_range_bounds::IpRangeBounds::validate_ip_range)]
        /// Range 1
        r1: String,

        #[clap(long, default_value_t = String::from("0-255"), forbid_empty_values = true, validator = ip::ip_range_bounds::IpRangeBounds::validate_ip_range)]
        /// Range 2
        r2: String,

        #[clap(long, default_value_t = String::from("0-255"), forbid_empty_values = true, validator = ip::ip_range_bounds::IpRangeBounds::validate_ip_range)]
        /// Range 3
        r3: String
    },
}

fn index(r0: &str, r1: &str, r2: &str, r3: &str, logger: &DummyLogger) -> std::io::Result<()> {
    logger.debug(format!("Indexing range: [{}].[{}].[{}].[{}]",r0,r1,r2,r3));

    let range = IpRange::new(
        IpRangeBounds::from_str(r0).unwrap(),
        IpRangeBounds::from_str(r1).unwrap(),
        IpRangeBounds::from_str(r2).unwrap(),
        IpRangeBounds::from_str(r3).unwrap(),
    );

    dbg!(range);

    // dbg!(range);
    return Ok(());
}

fn main() {
    let args = Arguments::parse();
    let logger = DummyLogger::new(args.verbosity);
    match args.cmd {
        SubCommand::Index { r0,r1,r2,r3 } => match index( &r0,&r1,&r2,&r3, &logger) {
            Ok(_) => println!("Done"),
            Err(e) => eprintln!("error in processing : {}", e),
        },
    }
}