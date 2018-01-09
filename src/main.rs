
extern crate futures;
extern crate tokio_core;
extern crate tokio_ping;
extern crate tokio_timer;
extern crate rusqlite;
extern crate chrono;

use std::process;
use std::env;
use std::net::IpAddr;
use std::time::Duration;
use std::path::PathBuf;

use futures::{Stream, Future};
use tokio_core::reactor::Core;
use chrono::Utc;
use tokio_timer::Timer;

mod db;
mod result;

use result::{PingResult, PingStatus};
use db::Database;

type BoxErr = Box<std::error::Error>;

fn main() {
    let args = args().unwrap_or_else(|e| {
        eprintln!("{}", e);
        eprintln!("Usage: netcheck target_ip database_path");
        process::exit(-2);
    });
    run(args).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(-1);
    })
}

fn run(args: Args) -> Result<(), BoxErr> {
    let ping_interval = Duration::from_secs(10);

    let mut database = Database::open(&args.path)?;

    println!("Pinging {}", args.address);
    let mut core = Core::new()?;

    let timer = Timer::default();
    let intervals = timer.interval(ping_interval).map_err(BoxErr::from);

    let pinger = tokio_ping::Pinger::new(&core.handle())?;
    let ident = 0;
    let mut sequence = 0;

    // On each interval, start a ping
    let interval_pings = intervals.and_then(|_| {
        let ping_future = pinger.ping(args.address, ident, sequence, ping_interval);
        sequence = sequence.wrapping_add(1);
        ping_future.map_err(BoxErr::from)
    });

    let done = interval_pings.map(|time_option| {
        let ping_status = time_option
            .map(PingStatus::Returned)
            .unwrap_or(PingStatus::Timeout);
        PingResult::new(Utc::now(), ping_status)
    }).for_each(|ping_result| {
        println!("{:?}", ping_result);
        database.save_result(ping_result).map_err(BoxErr::from)
    });

    core.run(done)?;

    Ok(())
}

struct Args {
    pub address: IpAddr,
    pub path: PathBuf,
}

fn args() -> Result<Args, BoxErr> {
    let address: IpAddr = env::args().nth(1)
        .ok_or(BoxErr::from("No IP address specified"))
        .and_then(|s| s.parse().map_err(From::from))?;
    let path = env::args_os().nth(2)
        .ok_or(BoxErr::from("No database path specified"))
        .map(PathBuf::from)?;
    Ok(Args {
        address,
        path,
    })
}
