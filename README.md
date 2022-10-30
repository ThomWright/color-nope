# Color nope

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/ThomWright/color-nope/ci)
![GitHub](https://img.shields.io/github/license/ThomWright/color-nope)
![Crates.io](https://img.shields.io/crates/v/color-nope)
![docs.rs](https://img.shields.io/docsrs/color-nope)

_Color? Nope. Nope nope nope._

Support for standard options to disable colors in the terminal.

An implementation of the [`NO_COLOR`](https://no-color.org/) standard, following the [Command Line Interface Guidelines](https://clig.dev/#output).

## Why

Different libraries do this their own way, often accessing global state (e.g. env vars), and sometimes assuming the output is always going to stdout.

What I wanted:

1. Support for [`NO_COLOR`](https://no-color.org/).

    This seems like a reasonable standard.

2. The ability to disable colors for stdout and stderr independently.

    Sometimes one might be connected to a tty, and the other piped to a file (for example).

3. To keep access to global state (e.g. env vars) exclusively inside my application (not in libraries).

    I have a strong belief that for the most part, libraries shouldn't touch global state. I, as the application developer, should be able to decide which environment variables I want to use and should be able to pass those to libraries.

    In this case, if I use two libraries to control color where one of them uses `NO_COLOR` and the other uses `CLICOLOR` then I'm pretty stuck unless I can take control back.

## Examples

Using the `from_env()` convenience function:

```rust
use color_nope::{ColorNope, Stream};

let enable_color = ColorNope::from_env().enable_color_for(Stream::Stdout);

println!("{enable_color}");
```

Or by passing in your own values:

```rust
use color_nope::{ColorNope, Stream, Force};

let enable_color = ColorNope::new(
    std::env::var_os("TERM"),
    std::env::var_os("NO_COLOR"),
    if std::env::args_os().any(|a| a == "--no-color") {
        Some(Force::Off)
    } else {
        None
    },
)
.enable_color_for(Stream::Stdout);

println!("{enable_color}");
```
