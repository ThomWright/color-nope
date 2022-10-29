# Color nope

An implementation of the [NO_COLOR](https://no-color.org/) standard, following the [Command Line Interface Guidelines](https://clig.dev/#output).

## Examples

Using the `from_env()` convenience function:

```rust
use color_nope::{ColorNope, Stream};

let enable_color = ColorNope::from_env().enable_color_for(Stream::Stdout);

println!("{enable_color}");
```

Or by passing in your own values:

```rust
use color_nope::{ColorNope, Stream};

let enable_color = ColorNope::new(
    std::env::var_os("TERM"),
    std::env::var_os("NO_COLOR"),
    std::env::args_os().any(|a| a == "--no-color"),
)
.enable_color_for(Stream::Stdout);

println!("{enable_color}");
```
