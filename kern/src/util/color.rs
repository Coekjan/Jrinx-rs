#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
#[repr(u8)]
pub enum ColorCode {
    RED = 31,
    GREEN = 32,
    YELLOW = 33,
    BLUE = 34,
    MAGENTA = 35,
    CYAN = 36,
    WHITE = 37,
}

macro_rules! with_color {
    ($color:expr, $restore_color:expr, $($arg:tt)*) => {{
        #[cfg(feature = "colorful")]
        {
            format_args!(
                "\u{1B}[{color}m{arg}\u{1B}[{restore}m",
                color = $color as u8,
                arg = format_args!($($arg)*),
                restore = $restore_color as u8,
            )
        }

        #[cfg(not(feature = "colorful"))]
        {
            $color as u8;
            $restore_color as u8;
            format_args!($($arg)*)
        }
    }};
}
pub(crate) use with_color;
