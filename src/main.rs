use std::path::PathBuf;

use chunk_type::ChunkType;
use clap::{Parser, Subcommand};

mod chunk;
mod chunk_type;
mod commands;
mod png;

#[derive(Parser)]
struct Args {
    path: PathBuf,
    #[command(subcommand)]
    command: PngMeCommand,
}

#[derive(Subcommand)]
enum PngMeCommand {
    Encode {
        chunk_type: ChunkType,
        message: String,
        output: Option<PathBuf>,
    },
    Decode {
        chunk_type: ChunkType,
    },
    Remove {
        chunk_type: ChunkType,
    },
    Print,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args.command {
        PngMeCommand::Encode {
            chunk_type,
            message,
            output,
        } => commands::encode(args.path, chunk_type, message, output),
        PngMeCommand::Decode { chunk_type } => commands::decode(args.path, &chunk_type),
        PngMeCommand::Remove { chunk_type } => commands::remove(args.path, &chunk_type),
        PngMeCommand::Print => commands::print(args.path),
    }
}
