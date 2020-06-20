use libps1::{Prompt, Theme};

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "binps1")]
struct Args {
    #[structopt(long)]
    theme: Option<Theme>,
}

/// This is a reference implementation of a shell prompt using libps1.
///
/// If you like one of the built-in themes, feel free to use this as
/// your shell prompt. If you prefer more customization, check out
/// the other examples.
fn main() {
    let args = Args::from_args();

    let prompt = match args.theme {
        Some(theme) => Prompt::with_theme(theme),
        None => Prompt::default(),
    };

    prompt.show();
}
