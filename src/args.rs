use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: PngMeArgs,
}

#[derive(Subcommand)]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Parser)]
pub struct EncodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
    pub message: String,
    pub output_file: Option<PathBuf>,
}

#[derive(Parser)]
pub struct DecodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
}

#[derive(Parser)]
pub struct RemoveArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
}

#[derive(Parser)]
pub struct PrintArgs {
    pub file_path: PathBuf,
}
