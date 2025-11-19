#![feature(lock_value_accessors)]
#![forbid(unsafe_code)]

mod app;
mod editor;
mod interconnect;
mod settings;
mod state;
mod utils;

#[derive(Debug, clap::Parser)]
struct Args {
    #[arg(index = 1)]
    game_dir: Option<std::path::PathBuf>,
}

fn main() -> bevy::app::AppExit {
    let mut args = <Args as clap::Parser>::parse();

    if let Some(path) = &args.game_dir {
        if path.ends_with("RPG_RT.ldb") {
            args.game_dir = Some(path.parent().unwrap().to_owned());
        }
    }

    app::run(args)
}
