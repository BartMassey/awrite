#[macro_export]
macro_rules! _async_try {
    ($stuff:tt) => {
        async {
            let mut wrapper = async || -> tokio::io::Result<_> {
                Ok($stuff)
            };
            wrapper().await
        }.await
    };
}

/// Equivalent to `writeln!()` but with `tokio` async I/O.
/// Expects the destination to obey `tokio::io::Write`.
/// Returns `tokio::io::Result<()>`.
#[macro_export]
macro_rules! awrite {
    ($dst:expr, $fmt:literal, $($arg:expr),*) => {{
        let mut buf: Vec<u8> = Vec::new();
        std::write!(buf, $fmt, $($arg),*).unwrap();
        $dst.write_all(&buf).await
    }};
    ($dst:expr, $fmt:literal) => {
        $dst.write_all($fmt.as_bytes()).await
    };
}

/// Equivalent to `writeln!()` but with `tokio` async I/O.
/// Writes `"\r\n"` as the line ending.
/// Expects the destination to obey `tokio::io::Write`.
/// Returns `tokio::io::Result<()>`.
#[macro_export]
macro_rules! awriteln {
    ($dst:expr, $fmt:literal, $($arg:expr),*) => {awrite::_async_try!{{
        awrite!($dst, $fmt, $($arg),*)?;
        awrite!($dst, "\r\n")?;
    }}};
    ($dst:expr, $fmt:literal) => {awrite::_async_try!{{
        awrite!($dst, $fmt)?;
        awrite!($dst, "\r\n")?;
    }}};
    ($dst:expr) => {
        awrite!($dst, "\r\n")
    };
}
