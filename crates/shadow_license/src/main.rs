use anyhow::Result;
use clap::Parser;
use duct::cmd;
use std::path::PathBuf;

#[derive(Parser)]
pub enum Args {
    #[clap(name = "generate")]
    Generate(GenerateArgs),
}

#[derive(Parser)]
pub struct GenerateArgs {
    #[clap(short, long, help = "the path to write the license")]
    pub output: PathBuf,
}

fn generate(generate_args: GenerateArgs) -> Result<()> {
    let private_key = cmd!("gpg", "--decrypt", "secrets/keys/license.private.rsa")
        .run()?
        .stdout;
    let private_key = String::from_utf8(private_key)?;
    let license = shadow_license::generate(&private_key)?;
    std::fs::write(generate_args.output, license)?;
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    match args {
        Args::Generate(generate_args) => generate(generate_args)?,
    };
    Ok(())
}