use libps1::{
    Color::{Green, Purple, Red, Yellow, RGB},
    Prompt,
};

/// This is a demonstration of a fully customized shell prompt
/// using libps1.
fn main() {
    Prompt {
        cwd_color: Purple,
        git_branch_color: RGB(0x17, 0xC8, 0xB0),
        git_status_clean_color: Green,
        git_status_unstaged_color: Red,
        git_status_staged_color: Yellow,
        // If you'd prefer not to specify all of the values, uncomment
        // the line below to fall back on a theme.
        //
        // use libps1::Theme::Solarized;
        // ..Prompt::with_theme(Solarized)
    }
    .show()
}
