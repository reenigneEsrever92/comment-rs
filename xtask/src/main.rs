use std::io::Write;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use clap::{AppSettings, Parser, Subcommand};
use comments_rs_graphql::RootQuery;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    GenerateSDL,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::GenerateSDL => generate_sdl(),
    }
}

fn generate_sdl() {
    let schema = Schema::build(RootQuery, EmptyMutation, EmptySubscription).finish();
    let dir = std::env::current_dir().unwrap().as_path().to_owned();
    let file_path = dir.join("comments-rs-graphql/schema.graphql");
    let mut file = std::fs::File::create(file_path)
        .expect("Could not create file");

    file.write_all(schema.sdl().as_bytes())
        .expect("Could not write schema");
}
