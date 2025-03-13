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
//! - TODO: Test using the `git ls-files` function as a backend instead.
//! - TODO: Implement `gls` functionality from the ground up (exercise).

use std::env;

fn main() {}
