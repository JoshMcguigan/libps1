use git2::{Repository, Status};
use std::{env, fmt::Display, path::Path};
use tico::tico;

pub use ansi_term::Color;
use ansi_term::Color::{Blue, Cyan, Green, Red, Yellow, RGB};

mod themes;
pub use themes::Theme;

pub struct Prompt {
    pub cwd_color: Color,
    /// Shorten the current working directory, by only printing
    /// the first character of each but the last directory.
    ///
    /// For exapmle, `/tmp/my_dir/foo` would become `/t/m/foo`.
    pub cwd_shorten_directories: bool,

    /// If provided, this string will be used in place of `/home/my_user`
    /// when printing the current working directory. For example, if
    /// this is set to `Some("~")`, then `/home/my_user/foo` will be
    /// printed as `~/foo`.
    pub cwd_shorten_home: Option<&'static str>,

    pub git_branch_color: Color,

    pub git_status_clean_color: Color,
    pub git_status_unstaged_color: Color,
    pub git_status_staged_color: Color,
    pub git_status_clean_icon: &'static str,
    pub git_status_unstaged_icon: &'static str,
    pub git_status_staged_icon: &'static str,

    /// Specify the string used to separate the prompt character
    /// from the preceding text. Some useful values for this are
    /// " " (single whitespace) or "\n" to draw the prompt character
    /// on a new line.
    pub prompt_char_separator: &'static str,
}

impl Default for Prompt {
    fn default() -> Self {
        Self {
            cwd_color: Cyan,
            cwd_shorten_directories: false,
            cwd_shorten_home: Some("~"),

            git_branch_color: Blue,

            git_status_clean_color: Green,
            git_status_unstaged_color: Red,
            git_status_staged_color: Yellow,
            git_status_clean_icon: "✓",
            git_status_unstaged_icon: "×",
            git_status_staged_icon: "±",

            prompt_char_separator: "\n",
        }
    }
}

impl Prompt {
    pub fn with_theme(theme: Theme) -> Self {
        match theme {
            Theme::Nord => {
                let nord_8 = RGB(0x88, 0xC0, 0xD0);
                let nord_9 = RGB(0x81, 0xA1, 0xC1);
                let nord_11 = RGB(0xBF, 0x61, 0x6A);
                let nord_13 = RGB(0xEB, 0xCB, 0x8B);
                let nord_14 = RGB(0xA3, 0xBE, 0x8C);

                Self {
                    cwd_color: nord_8,
                    git_branch_color: nord_9,
                    git_status_clean_color: nord_14,
                    git_status_unstaged_color: nord_11,
                    git_status_staged_color: nord_13,
                    ..Self::default()
                }
            }
            Theme::Solarized => Self {
                cwd_color: RGB(0x2A, 0xA1, 0x98),
                git_branch_color: RGB(0x26, 0x8B, 0xD2),
                git_status_clean_color: RGB(0x58, 0x6E, 0x75),
                git_status_unstaged_color: RGB(0xCB, 0x4B, 0x16),
                git_status_staged_color: RGB(0x65, 0x7B, 0x83),
                ..Self::default()
            },
        }
    }

    pub fn show(self) {
        let cwd = {
            let cwd = self.cwd().unwrap_or_else(|| "".into());

            apply_color(cwd, self.cwd_color)
        };

        let vcs_status = vcs_status();

        let prompt_char = get_char();

        match vcs_status {
            Some((branch, status)) => {
                let branch = apply_color(branch, self.git_branch_color);
                let status = match status {
                    GitStatus::Clean => {
                        apply_color(self.git_status_clean_icon, self.git_status_clean_color)
                    }
                    GitStatus::Unstaged => apply_color(
                        self.git_status_unstaged_icon,
                        self.git_status_unstaged_color,
                    ),
                    GitStatus::Staged => {
                        apply_color(self.git_status_staged_icon, self.git_status_staged_color)
                    }
                };
                println!(
                    "{cwd} {branch} {status}{separator}{pchar} ",
                    cwd = cwd,
                    branch = branch,
                    status = status,
                    separator = self.prompt_char_separator,
                    pchar = prompt_char,
                )
            }
            None => println!(
                "{cwd}{separator}{pchar} ",
                cwd = cwd,
                separator = self.prompt_char_separator,
                pchar = prompt_char,
            ),
        };
    }

    fn cwd(&self) -> Option<String> {
        let path_env = env::current_dir().ok()?;
        let mut path = format!("{}", path_env.display());

        if let Some(user_desired_home_str) = self.cwd_shorten_home {
            let home_dir = env::var("HOME").ok()?;
            let home_dir_ext = format!("{}/", home_dir);

            if (path == home_dir) || path.starts_with(&home_dir_ext) {
                path = path.replacen(&home_dir, user_desired_home_str, 1);
            }
        }

        if self.cwd_shorten_directories {
            path = tico(&path);
        }

        Some(path)
    }
}

fn apply_color(text: impl Display, color: Color) -> String {
    let start_color_code = 1 as char;
    let end_color_code = 2 as char;
    format!(
        "{}{}{}{}{}{}{}",
        start_color_code,
        color.prefix(),
        end_color_code,
        text,
        start_color_code,
        color.suffix(),
        end_color_code,
    )
}

fn get_char() -> &'static str {
    const ROOT_UID: u32 = 0;
    let uid = unsafe { libc::geteuid() };

    if uid == ROOT_UID {
        "#"
    } else {
        "$"
    }
}

enum GitStatus {
    Clean,
    /// Has some unstaged changed.
    Unstaged,
    /// All changes staged.
    Staged,
}

fn vcs_status() -> Option<(String, GitStatus)> {
    let current_dir = env::var("PWD").ok()?;

    let repo = {
        let mut repo: Option<Repository> = None;
        let current_path = Path::new(&current_dir[..]);
        for path in current_path.ancestors() {
            if let Ok(r) = Repository::open(path) {
                repo = Some(r);
                break;
            }
        }

        repo?
    };

    let mut commit_dist: String = "".into();
    if let Some((ahead, behind)) = get_ahead_behind(&repo) {
        if ahead > 0 {
            commit_dist.push_str(" ↑");
        }
        if behind > 0 {
            commit_dist.push_str(" ↓");
        }
    }

    let reference = repo.head().ok()?;

    let branch = if reference.is_branch() {
        format!("{}{}", reference.shorthand()?, commit_dist)
    } else {
        let commit = reference.peel_to_commit().ok()?;
        let id = commit.id();

        format!("{:.6}{}", id, commit_dist)
    };

    let mut repo_status = GitStatus::Clean;

    for file in repo.statuses(None).ok()?.iter() {
        match file.status() {
            // STATE: unstaged (working tree modified)
            Status::WT_NEW
            | Status::WT_MODIFIED
            | Status::WT_DELETED
            | Status::WT_TYPECHANGE
            | Status::WT_RENAMED => {
                repo_status = GitStatus::Unstaged;
                break;
            }
            // STATE: staged (changes added to index)
            Status::INDEX_NEW
            | Status::INDEX_MODIFIED
            | Status::INDEX_DELETED
            | Status::INDEX_TYPECHANGE
            | Status::INDEX_RENAMED => {
                repo_status = GitStatus::Staged;
            }
            // STATE: committed (changes have been saved in the repo)
            _ => {}
        }
    }

    Some((branch, repo_status))
}

fn get_ahead_behind(r: &Repository) -> Option<(usize, usize)> {
    let head = r.head().ok()?;
    if !head.is_branch() {
        return None;
    }

    let head_name = head.shorthand()?;
    let head_branch = r.find_branch(head_name, git2::BranchType::Local).ok()?;
    let upstream = head_branch.upstream().ok()?;
    let head_oid = head.target()?;
    let upstream_oid = upstream.get().target()?;

    r.graph_ahead_behind(head_oid, upstream_oid).ok()
}
