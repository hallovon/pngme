use clap::Parser;
use pngme::{
    args::{Cli, PngMeArgs},
    commands, Result,
};

fn main() -> Result<()> {
    match &Cli::parse().command {
        PngMeArgs::Encode(args) => commands::encode(args),
        PngMeArgs::Decode(args) => commands::decode(args),
        PngMeArgs::Remove(args) => commands::remove(args),
        PngMeArgs::Print(args) => commands::print(args),
    }
}
