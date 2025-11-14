# awrite: write and writeln for tokio
Bart Massey 2025

The macros in this library implement equivalent
functionality to the `write!` and `writeln!` macros for the
`tokio` crate, under the names `awrite!` and `awriteln!`.

The macros write to a `Vec<u8>` internally, then
`tokio::write_all()` the contents of that `Vec`.

The `awriteln!` macro uses `\r\n` as the line ending, for
compatibility with things that need that.

Both macros return `tokio::io::Result<()>`.

## Acknowledgements

Thanks to
<https://users.rust-lang.org/t/equivalent-of-writeln-for-tokio/69002/5>
for the starting idea.
