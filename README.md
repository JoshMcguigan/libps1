# libps1

libps1 is an experimental shell prompt for power users. There are many great shell prompt prjects in existence, but this one is different. While nearly every other option uses some kind of configuration file (toml, yaml, etc), libps1 is intended to be used as a library, so it can be customized and documented using the incredible tooling developed by the Rust ecosystem. No more wondering what other options are available, or if you are spelling things correctly, your IDE and the compiler will guide you.

![screenshot](https://user-images.githubusercontent.com/22216761/85191296-2d58d100-b273-11ea-99b7-45653b579d49.png)

## Getting Started

### Option 1 - binps1

binps1 is the reference shell prompt implementation, using libps1.

```shell
cargo install --force --path . --example binps1
```

```shell
# .bashrc

# default color scheme
PS1='$(binps1)'

# or pick one of the available custom color schemes
PS1='$(binps1 --theme solarized)'
```

### Option 2 - Make it your own!

```shell
cargo new --bin my_shell # pick your own name here
```

```toml
# cargo.toml
[dependencies]
libps1 = { git = "https://github.com/JoshMcguigan/libps1" }
```

```rust
# main.rs

use libps1::{
    Color::{Green, Purple, Red, Yellow, RGB},
    Prompt,
};

/// This is a demonstration of a fully customized shell prompt
/// using libps1.
fn main() {
    Prompt {
        cwd_color: Purple,
        cwd_shorten_directories: false,
        cwd_shorten_home: Some("⌂"),

        git_branch_color: RGB(0x17, 0xC8, 0xB0),

        git_status_clean_color: Green,
        git_status_unstaged_color: Red,
        git_status_staged_color: Yellow,
        git_status_clean_icon: "➖",
        git_status_unstaged_icon: "❌",
        git_status_staged_icon: "➕",
        // If you'd prefer not to specify all of the values, uncomment
        // the line below to fall back on a theme. And add the import
        // to the top of the file.
        //
        // use libps1::Theme::Solarized;
        // ..Prompt::with_theme(Solarized)
    }
    .show()
}
```

Then `cargo install` your binary and setup your `.bashrc` as shown in Option 1.

## Related Projects

libps1 is a fork of [pista](https://github.com/NerdyPepper/pista) by [@NerdyPepper](https://github.com/NerdyPepper).
