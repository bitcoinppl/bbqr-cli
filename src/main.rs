pub mod cli;
pub mod split;

use clap::{Parser, Subcommand};
use eyre::Result;

#[derive(Parser, Debug)]
#[clap(name = "BBQr", author, version, about, long_about = None)]
#[command(styles=cli::get_styles())]
#[clap(args_override_self = true, arg_required_else_help = true)]
struct BbqrCli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Merge bbqr splits (currently not implemented)
    Merge(MergeArgs),

    /// Split input
    Split(SplitArgs),
}

#[derive(Parser, Debug)]
pub struct SplitArgs {
    /// Input string or file path
    #[arg(name = "INPUT_DATA_OR_FILE_PATH")]
    input: String,
    /// Minimum number of splits
    #[arg(short = 'm', long, default_value_t = 1)]
    min_splits: usize,
    /// Maximum number of splits
    #[arg(short = 'M', long, default_value_t = 138)]
    max_splits: usize,

    /// Version to use for splitting
    #[arg(short, long)]
    version: Option<u16>,

    /// Min version to use for splitting
    #[arg(short = 'x', long, default_value_t = 1)]
    min_version: u16,

    /// Max version to use for splitting
    #[arg(short = 'X', long, default_value_t = 20)]
    max_version: u16,

    /// Output directory
    /// If not provided, will show the QR codes in the terminal
    #[arg(short, long)]
    output: Option<String>,
}

#[derive(Parser, Debug)]
struct MergeArgs {
    /// Input string, file path, or use stdin if not provided
    input: Option<String>,
}

fn main() -> Result<()> {
    env_logger::init();
    color_eyre::install()?;

    let args = BbqrCli::parse();

    match args.command {
        Commands::Split(split_args) => split::run(split_args)?,
        Commands::Merge(_) => unimplemented!(),
    };

    Ok(())
}
