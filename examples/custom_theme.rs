use libps1::{
    Color::{Green, Purple, Red, Yellow, RGB},
    Prompt,
};

/// This is a demonstration of a fully customized shell prompt
/// using libps1.
fn main() {
    Prompt {
        // Colors
        cwd_color: Purple,
        git_branch_color: RGB(0x17, 0xC8, 0xB0),
        git_status_clean_color: Green,
        git_status_unstaged_color: Red,
        git_status_staged_color: Yellow,

        // Icons
        git_status_clean_icon: "➖",
        git_status_unstaged_icon: "❌",
        git_status_staged_icon: "➕",

        // Current working directory
        shorten_cwd: false,
        shorten_home_cwd: Some("⌂"),
        // If you'd prefer not to specify all of the values, uncomment
        // the line below to fall back on a theme. And add the import
        // to the top of the file.
        //
        // use libps1::Theme::Solarized;
        // ..Prompt::with_theme(Solarized)
    }
    .show()
}
