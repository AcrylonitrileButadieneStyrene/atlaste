#![feature(lock_value_accessors)]
#![forbid(unsafe_code)]

mod app;
mod editor;
mod fonts;
mod state;
mod ui;

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
    } else {
        rfd::MessageDialog::new()
            .set_title("Atlaste")
            .set_description("Please select the RPG_RT.ldb file for the game.")
            .set_buttons(rfd::MessageButtons::Ok)
            .show();
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("RPG_RT.ldb", &["ldb"])
            .pick_file()
        {
            args.game_dir = Some(path.parent().unwrap().to_owned());
        } else {
            std::process::exit(0);
        }
    }

    app::run(args)
}
