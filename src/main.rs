//! This crate provides a command-line tool `acacls` alternative to `gls`
//! By default, `acacls` does not show both "hidden" files and "ignored" files.
//!
//! Filenames of hidden and ignored files are written in files following the
//! ['.gitignore' pattern](https://git-scm.com/docs/gitignore#_pattern_format).
//!
//! # Options
//!
//! `-a`, `--show-hidden`
//!     Show hidden files.
//! `-A`, `--show-ignored`
//!     Show all ignored files (includes all hidden files).
//! `--all`
//!     TODO: Precise documentation and funcionality.
//! `--almost-all`
//!     TODO: Precise documentation and funcionality.
//!
//! All other options are passed directly to `gls`.
//!
//! # Hidden Files
//!
//! Filenames of hidden files are listed locally in a '.hidden' file
//! or globally in the file at the path in the LS_GLOBAL_HIDDEN environment variable.
//!
//! Filenames beginning with a '.' are considered hidden unless specifically excluded
//! in a '.hidden' file.
//! Hidden files are a superset of ignored files.
//!
//! # Ignored Files
//!
//! Filenames of ignored files are listed locally in a '.ignore' file
//! or globally in the file at the path in the LS_GLOBAL_IGNORE environment variable.
//!
//! The implicit '.' and '..' files in every directory are ignored by default.
//! Ignored files are a subset of hidden files.
//!
//! # Plans for `acacls`
//!
//! - TODO: Implement basic functionality and tests.
//! - TODO: Support and benchmark looking in parent directories for .hidden and .ignore files.
//! - TODO: Documentation and functionality for `--all` and `--almost-all`.
//! - TODO: Add ! functionality to the pattern matching.
//! - TODO: Implement `gls` functionality from the ground up (exercise).

use std::cmp::max;
use std::env;
use std::ops;

/// Behaviors of the `acacls` command line tool.
///
/// Keep in mind that ignored files are a subset of hidden files.
#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Debug)]
enum HideOption {
    HideHidden = 0,
    HideIgnored = 1,
    ShowAll = 2,
}

impl ops::Add<HideOption> for HideOption {
    type Output = HideOption;

    /// Return which of the two hide options shows the most files.
    fn add(self, rhs: HideOption) -> HideOption {
        max(self, rhs)
    }
}

impl ops::AddAssign<HideOption> for HideOption {
    fn add_assign(&mut self, rhs: HideOption) {
        *self = *self + rhs
    }
}

/// Parse one or multiple flags on the command line.
///
/// Flags, or short options, can be passed individually or group together.
/// This method searches for 'a' or 'A', removes that from the string,
/// and returns the corresponding hide option.
///
/// The returned option is None if there are no other flags in the string,
/// and Some with the remaining flags if there are.
///
/// The starting '-' is optional in the argument, but the returned String
/// will always contain a starting '-'.
///
/// This parsing method generally won't work when flags have parameters,
/// but this is not the case for 'gls'.
fn parse_flags(flags: &str) -> (Option<String>, HideOption) {
    let mut gls_flags = String::from("-");
    let mut hide_option = HideOption::HideHidden;
    for c in flags.chars() {
        match c {
            'a' => hide_option += HideOption::HideIgnored,
            'A' => hide_option += HideOption::ShowAll,
            '-' => (),
            _ => gls_flags.push(c),
        }
    }

    let gls_flag_opt = if gls_flags.len() > 1 {
        Some(gls_flags)
    } else {
        None
    };

    (gls_flag_opt, hide_option)
}

/// Parse a list of command line arguments.
///
/// Returns a 2-tuple `(gls_args, hide_option)`.
/// `gls_args` contains the command line arguments that will be passed to `gls`.
/// `hide_options` documents the option passed indicating which files should appear.
///
/// This function parses all items in `args`, so the first command line argument
/// should generally not be passed in as part of `args`.
fn parse_args(args: &[String]) -> (Vec<String>, HideOption) {
    let mut gls_args = Vec::new();
    let mut hide_option = HideOption::HideHidden;

    for arg in args {
        match arg.as_str() {
            "--show-ignored" => hide_option += HideOption::ShowAll,
            "--show-hidden" => hide_option += HideOption::HideIgnored,
            s if s.starts_with("-") && !s.starts_with("--") => {
                let (gls_flag_opt, hide_flag) = parse_flags(s);
                hide_option += hide_flag;
                if let Some(gls_flags) = gls_flag_opt {
                    gls_args.push(gls_flags);
                }
            }
            _ => gls_args.push(arg.into()),
        }
    }
    (gls_args, hide_option)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let (gls_args, hide_option) = parse_args(&args[1..]);
    println!("{:?}", gls_args);
    println!("{:?}", hide_option);
    // Search for global and local hidden and ignore files.
    // Collate relevant --ignore options if the file is in the directory.
    // If -a, pass in -A to gls and --ignore with hidden files.
    // If -A, pass in -a to gls and no --ignore.
    // Otherwise, pass in all --ignore options.
}

// TODO: Write tests for the argument parsing functions.
