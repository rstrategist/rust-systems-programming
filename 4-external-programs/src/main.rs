/// # External-programs
///
/// A `df` wrapper in Rust.
///
/// This program provides a command-line interface to display disk space usage
/// information, similar to the Unix `df` command. It supports various options
/// such as increased verbosity, logging to a file, and enabling debug mode.
/// 
/// ## Usage Examples
///
/// Run the program with the `info` subcommand:
/// ```sh
/// cargo run -- info
/// ```
///
/// Run the program with increased verbosity:
/// ```sh
/// cargo run -- -v info
/// ```
///
/// Enable logging to a file:
/// ```sh
/// cargo run -- --log-file info
/// ```
///
/// Enable debug mode:
/// ```sh
/// cargo run -- --debug info
/// ```
use clap::{ArgAction, Parser};
use env_logger::{Builder, Target};
use external_programs::run_df;
use log::LevelFilter;
use std::fs::OpenOptions;

// Command-line options for the program.
#[derive(Parser)]
#[command(
    name = "external-programs",
    version = "0.0.1",
    about = "df wrapper in Rust"
)]
struct Opts {
    /// Verbosity level (can be used multiple times for increased verbosity)
    #[clap(short, long, action = ArgAction::Count)]
    verbose_level: u8,

    // Enable logging to a file
    #[clap(long, help = "Enable logging to a file")]
    log_file: bool,

    // Enable debug mode
    #[clap(short, long, env = "RDF_DEBUG")]
    debug: bool,

    // Subcommand to execute
    #[clap(subcommand)]
    cmd: Command,
}

// Subcommands for the program
#[derive(Parser)]
enum Command {
    // Get information about a device
    #[clap(name = "info", about = "Get information about a device")]
    Info(InfoOpts),
}

// Options for the `info` subcommand
#[derive(Parser)]
struct InfoOpts {
    #[clap(help = "Path to query", default_value = "")]
    path: String,
}

fn main() {
    // Parse command-line arguments
    let opts = Opts::parse();

    let mut builder = Builder::new();
    builder.filter_level(LevelFilter::Debug);

    // If logging to a file is enabled, set up the file logger
    if opts.log_file {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("rdf.log")
            .unwrap();
        builder.target(Target::Pipe(Box::new(file)));
    }

    builder.init();

    // Log debug message if debug mode is enabled
    if opts.debug {
        log::debug!("Debug mode enabled");
    }

    // Execute the appropriate subcommand
    match opts.cmd {
        Command::Info(info_opts) => {
            // Handle verbosity levels
            match opts.verbose_level {
                0 => {
                    // Quiet mode
                }
                1 => {
                    println!("Running in verbose mode level 1");
                }
                2 => {
                    println!("Running in verbose mode level 2");
                }
                3 | _ => {
                    println!("Running in verbose mode level 3");
                }
            }

            // Run the `df` command and print the output
            let output = serde_json::to_string_pretty(&run_df(&info_opts.path)).unwrap();
            println!("{}", output);
        }
    }
}
