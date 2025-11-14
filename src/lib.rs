/*! Macros to use instead of `write!` and `writeln` in Tokio.

Thanks to
<https://users.rust-lang.org/t/equivalent-of-writeln-for-tokio/69002/5>
for the starting idea.
*/

pub(crate) mod _async_try {
    #[doc(hidden)]
    #[allow(unused)]
    macro_rules! async_try {
        ($stuff:tt) => {
            async {
                let mut wrapper = async || -> tokio::io::Result<_> {
                    Ok($stuff)
                };
                wrapper().await
            }.await
        };
    }
}

/// Equivalent to `writeln!()` but with `tokio` async I/O.
/// Expects the destination to obey `tokio::io::Write`.
/// Returns `tokio::io::Result<()>`.
#[macro_export]
macro_rules! awrite {
    ($dst:expr, $fmt:literal, $($arg:expr),*) => {{
        let mut buf: Vec<u8> = Vec::new();
        std::write!(buf, $fmt, $($arg),*).unwrap();
        tokio::io::AsyncWriteExt::write_all($dst, &buf).await
    }};
    ($dst:expr, $fmt:literal) => {
        tokio::io::AsyncWriteExt::write_all($dst, $fmt.as_bytes()).await
    };
}

/// Equivalent to `writeln!()` but with `tokio` async I/O.
/// Writes `"\r\n"` as the line ending.
/// Expects the destination to obey `tokio::io::Write`.
/// Returns `tokio::io::Result<()>`.
#[macro_export]
macro_rules! awriteln {
    ($dst:expr, $fmt:literal, $($arg:expr),*) => {$crate::async_try!{{
        awrite!($dst, $fmt, $($arg),*)?;
        awrite!($dst, "\r\n")?;
    }}};
    ($dst:expr, $fmt:literal) => {$crate::async_try!{{
        awrite!($dst, $fmt)?;
        awrite!($dst, "\r\n")?;
    }}};
    ($dst:expr) => {
        awrite!($dst, "\r\n")
    };
}
