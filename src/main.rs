use args::Args;
use log::LevelFilter;

mod args;
mod app;

type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    if let Err(err) = Args::parse().and_then(try_main) {
        eprintln!("{}", err);
        std::process::exit(2);
    }
}

fn try_main(args: Args) -> Result<()> {
    let log_level = if args.quiet() {
        LevelFilter::Off
    } else if args.debug() {
        LevelFilter::Debug
    } else if args.verbose() {
        LevelFilter::Info
    } else {
        LevelFilter::Warn
    };
    env_logger::builder()
        .filter_level(log_level)
        .format_timestamp(None)
        .init();

    println!("Using file '{}' as input", args.input());
    process_input()
}

fn process_input() -> Result<()> {
    log::info!("***ENTITY RECOGNITION: START!***");
    log::info!("***   1) Multiwords***");
    log::info!("***ENTITY RECOGNITION: DONE!***");
    Ok(())
}
