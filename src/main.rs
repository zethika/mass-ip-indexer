use clap::{Parser, Subcommand};
use logger::DummyLogger;
use std::collections::VecDeque;
use std::str::FromStr;
use crate::ip::ip_range::IpRange;
use crate::ip::ip_range_bounds::IpRangeBounds;
use std::time::{Instant};

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

    let mut range = IpRange::new(
        IpRangeBounds::from_str(r0).unwrap(),
        IpRangeBounds::from_str(r1).unwrap(),
        IpRangeBounds::from_str(r2).unwrap(),
        IpRangeBounds::from_str(r3).unwrap(),
    );

    let full = Instant::now();
    let mut batch_nr: u32 = 0;
    let mut generated: u64 = 0;
    while range.has_more_batches() {
        logger.debug(format!("Batch: {}", batch_nr));

        let batch = range.generate_next_batch(100000);

        generated = generated + batch.len() as u64;

        let next_duration = full.elapsed();
        let pr_milli: u32 = generated as u32/next_duration.as_millis() as u32;
        logger.debug(format!("  Generated: {} ips at {} pr. milli", generated,pr_milli));
        batch_nr = batch_nr + 1;
    }

    let pr_second: f32 = full.elapsed().as_millis() as f32 / 1000 as f32;
    logger.debug(format!("Finished in {} seconds", pr_second));
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