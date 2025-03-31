#![feature(lock_value_accessors)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]

mod app;
mod lcf_asset_loader;
mod ui;

#[derive(Debug, clap::Parser)]
struct Args {
    #[arg(long)]
    game_dir: Option<std::path::PathBuf>,
}

fn main() {
    let args = <Args as clap::Parser>::parse();

    app::run(args);
}
