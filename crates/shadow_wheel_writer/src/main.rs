use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use wheel_writer::WheelWriter;

#[derive(Parser)]
struct Args {
    #[clap(long, help = "The supported ABI version")]
    abi: Option<String>,
    #[clap(long, help = "Optional build number. Must start with a digit.")]
    build_tag: Option<String>,
    #[clap(long, help = "Name of the distribution to package")]
    distribution: String,
    #[clap(long, help = "Location of the TOML file defining the metadata.")]
    metadata: PathBuf,
    #[clap(long, help = "Destination directory for the wheel file output.")]
    output: PathBuf,
    #[clap(
        long,
        alias = "package",
        help = "Directory containing the Python package to include in the wheel."
    )]
    packages: Vec<PathBuf>,
    #[clap(long, help = "Platform string this wheel supports.")]
    platform: Option<String>,
    #[clap(long, help = "Python environment and supported version tag")]
    python: String,
    #[clap(long, help = "Distribution version.")]
    version: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    WheelWriter {
        abi: args.abi,
        build_tag: args.build_tag,
        distribution: args.distribution,
        metadata_toml_path: args.metadata,
        packages: args.packages,
        platform: args.platform,
        python_tag: args.python,
        version: args.version,
    }
    .write(args.output)?;
    Ok(())
}
