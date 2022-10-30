/*!
Support for standard options to disable colors in the terminal.

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
/// use color_nope::{ColorNope, Stream, Force};
///
/// assert_eq!(
///     ColorNope::new(
///         std::env::var_os("TERM"),
///         std::env::var_os("NO_COLOR"),
///         if std::env::args_os().any(|a| a == "--no-color") {
///             Some(Force::Off)
///         } else {
///             None
///         },
///     )
///     .enable_color_for(Stream::Stdout),
///     false
/// );
/// ```
#[derive(Clone, Debug)]
pub struct ColorNope {
    term_env: Option<OsString>,
    no_color_env: Option<OsString>,
    force_color: Option<Force>,
}

impl ColorNope {
    /// Create a new instance without touching the environment.
    ///
    /// [`ColorNope`] considers the `TERM` and `NO_COLOR` environmental
    /// variables (`term_env` and `no_color_env` respectively).
    ///
    /// These values can be overridden by using `force_color`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use color_nope::ColorNope;
    /// ColorNope::new(
    ///     std::env::var_os("TERM"),
    ///     std::env::var_os("NO_COLOR"),
    ///     None
    /// );
    /// ```
    pub fn new(
        term_env: Option<OsString>,
        no_color_env: Option<OsString>,
        force_color: Option<Force>,
    ) -> ColorNope {
        ColorNope {
            term_env,
            no_color_env,
            force_color,
        }
    }

    /// Uses the `TERM` and `NO_COLOR` environmental variables.
    pub fn from_env() -> ColorNope {
        ColorNope {
            term_env: std::env::var_os("TERM"),
            no_color_env: std::env::var_os("NO_COLOR"),
            force_color: None,
        }
    }

    /// Should color be enabled for the target stream?
    pub fn enable_color_for(&self, stream: Stream) -> bool {
        match self.force_color {
            Some(force) => force.enable_color(),
            None => {
                atty::is(stream.into())
                    && term_allows_color(self.term_env.as_ref())
                    && self.no_color_env.is_none()
            }
        }
    }
}

/// Output streams.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Stream {
    #[allow(missing_docs)]
    Stdout,
    #[allow(missing_docs)]
    Stderr,
}
impl From<Stream> for atty::Stream {
    fn from(s: Stream) -> Self {
        match s {
            Stream::Stdout => atty::Stream::Stdout,
            Stream::Stderr => atty::Stream::Stderr,
        }
    }
}

/// Override other settings to force colors on or off.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Force {
    #[allow(missing_docs)]
    On,
    #[allow(missing_docs)]
    Off,
}
impl Force {
    fn enable_color(&self) -> bool {
        use Force::*;
        match self {
            On => true,
            Off => false,
        }
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
