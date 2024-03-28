use std::{thread::sleep, time::Duration};

use clap::Parser;
use swayipc::{Connection, Fallible};

/// program to resize applications smoothly
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// mode (shrink | grow)
    #[arg(required = true)]
    mode: String,

    /// direction (width | height)
    #[arg(required = true)]
    direction: String,

    /// amount in pixel
    #[arg(required = true, default_value = "10")]
    amount: i32,
}

fn main() -> Fallible<()> {
    let args: Args = Args::parse();
    let command = format!("resize {} {} {}px", args.mode, args.direction, args.amount);
    let mut connection = Connection::new()?;

    loop {
        let response = connection.run_command(command.as_str())?;
        for r in response {
            r?;
        }
        sleep(Duration::from_millis(5));
    }
}

