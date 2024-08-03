#[macro_export]
macro_rules! format_context {
    ($($arg:tt)*) => {{
        let res = format!($($arg)*);
        format!("[{}:{}] {}", file!(), line!(), res)
    }};
}

#[macro_export]
macro_rules! format_error {
    ($($arg:tt)*) => {{
        let res = format!($($arg)*);
        anyhow::anyhow!("[{}:{}] {}", file!(), line!(), res)
    }};
}

