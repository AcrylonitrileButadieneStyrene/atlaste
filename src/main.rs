#![feature(lock_value_accessors)]
#![forbid(unsafe_code)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]
#![allow(dead_code)]

mod app;
mod fonts;
mod lcf_asset_loader;
mod state;
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
