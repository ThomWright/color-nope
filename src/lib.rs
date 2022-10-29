/*!
An implementation of the [NO_COLOR](https://no-color.org/) standard, following
the [Command Line Interface Guidelines](https://clig.dev/#output).

## Usage

See [`ColorNope`] for usage examples.
*/

#![deny(missing_docs)]

#[cfg(doctest)]
use doc_comment::doctest;
#[cfg(doctest)]
doctest!("../README.md");

use std::ffi::OsString;

pub use atty::Stream;

/// Decides whether color should be enabled, based on the environment and the
/// target stream.
///
/// Assumes color is enabled by default, unless indicated otherwise.
///
/// # Examples
///
/// Can be created using the `from_env()` convenience function:
///
/// ```rust
/// use color_nope::{ColorNope, Stream};
///
/// assert_eq!(
///     ColorNope::from_env().enable_color_for(Stream::Stdout),
///     false
/// );
/// ```
///
/// Or by passing in your own values:
///
/// ```rust
/// use color_nope::{ColorNope, Stream};
///
/// assert_eq!(
///     ColorNope::new(
///         std::env::var_os("TERM"),
///         std::env::var_os("NO_COLOR"),
///         std::env::args_os().any(|a| a == "--no-color"),
///     )
///     .enable_color_for(Stream::Stdout),
///     false
/// );
/// ```
#[derive(Clone, Debug)]
pub struct ColorNope {
    term_env: Option<OsString>,
    no_color_env: Option<OsString>,
    no_color_flag: bool,
}

impl ColorNope {
    /// Create a new instance without touching the environment.
    ///
    /// # Arguments
    ///
    /// - `term_env` – `TERM` environmental variable.
    /// - `no_color_env` – `NO_COLOR` environmental variable.
    /// - `no_color_flag` - indicates the presence of a command-line flag
    /// overriding the environment, e.g. `--no-color`.
    pub fn new(
        term_env: Option<OsString>,
        no_color_env: Option<OsString>,
        no_color_flag: bool,
    ) -> ColorNope {
        ColorNope {
            term_env,
            no_color_env,
            no_color_flag,
        }
    }

    /// Checks the `TERM` and `NO_COLOR` environmental variables, as well as
    /// the presence of a `--no-color` flag.
    pub fn from_env() -> ColorNope {
        ColorNope {
            term_env: std::env::var_os("TERM"),
            no_color_env: std::env::var_os("NO_COLOR"),
            no_color_flag: std::env::args_os().any(|a| a == "--no-color"),
        }
    }

    /// Should color be enabled for the target stream?
    pub fn enable_color_for(&self, stream: atty::Stream) -> bool {
        atty::is(stream)
            && term_allows_color(self.term_env.as_ref())
            && self.no_color_env.is_none()
            && !self.no_color_flag
    }
}

// These next functions are shamelessly stolen from [termcolor](https://github.com/BurntSushi/termcolor).

#[cfg(not(windows))]
fn term_allows_color(term: Option<&OsString>) -> bool {
    match term {
        // If TERM isn't set, then we are in a weird environment that
        // probably doesn't support colors.
        None => false,
        Some(v) => v != "dumb",
    }
}

#[cfg(windows)]
fn term_allows_color(term: Option<&OsString>) -> bool {
    // On Windows, if TERM isn't set, then we shouldn't automatically
    // assume that colors aren't allowed. This is unlike Unix environments
    // where TERM is more rigorously set.
    if let Some(v) = term {
        v != "dumb"
    } else {
        true
    }
}
