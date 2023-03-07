//! # owo_farver
extern crate owo_colors;
#[allow(unused_imports)]
use owo_colors::OwoColorize;

// pub mod farve {
/// Pad a string with bold-white square brackets
/// `( "s" , 0 | 1 | 2 | _ ) -> "[s]"`
///
/// # Examples
/// ```
/// use owo_colors::OwoColorize;
/// use owo_farver::brace;
///
/// let i0 = brace!("INFO");
/// ```
#[macro_export]
macro_rules! brace {
    ($msg:expr) => {
        format!("{}{}{}", '[', $msg, ']')
    };

    ($level:expr, $brace_color:expr) => {
        match $brace_color {
            0 => format!(
                "{}{}{}",
                '['.black().dimmed().bold(),
                $level,
                ']'.black().dimmed().bold()
            ),
            1 => format!(
                "{}{}{}",
                '['.bright_black().bold(),
                $level,
                ']'.bright_black().bold()
            ),
            2 => format!("{}{}{}", '['.white().bold(), $level, ']'.white().bold()),
            _ => format!("{}{}{}", '[', $level, ']'),
        }
    };
}

/// Println to stdout with braces
/// # Examples
/// ```
/// use owo_colors::OwoColorize;
/// use owo_farver::prettyln;
///
/// prettyln!("info", "The weather is nice today.");
/// prettyln!("warn".yellow(), "I almost couldn't, but I did it!");
/// ```
#[macro_export]
macro_rules! prettyln {
    ($level:expr, $msg:expr, $brace_color:expr) => {
        println!("{} {}", owo_farver::brace!($level, $brace_color), $msg)
    };
    ($level:expr, $msg:expr) => {
        println!("{} {}", owo_farver::brace!($level), $msg)
    };
    ($level:expr) => {
        println!("{} {}", owo_farver::brace!($level))
    };
}

/// Eprintln to stderr with braces
#[macro_export]
macro_rules! eprettyln {
    ($level:expr, $msg:expr, $brace_color:expr) => {
        eprintln!("{} {}", brace!($level, $brace_color), $msg)
    };
    ($level:expr, $msg:expr) => {
        eprintln!("{} {}", brace!($level), $msg)
    };
    ($level:expr) => {
        eprintln!("{} {}", brace!($level))
    };
}

/// Create a new function with a custom name and prefix.
/// # Examples
/// ```
/// use owo_colors::OwoColorize;
/// use owo_farver::farve;
///
/// farve!(silly, "silly 😋".white().bold());
/// farve!(info);
/// farve!(warn, "warn".yellow().underline());
///
/// fn main() {
///   silly("Hello, world!");
///   info("The weather is nice today.");
///   warn("I almost couldn't, but I did it!");
/// }
/// ```
/// # Output (imagine color)
/// ```log
/// [silly 😋] Hello, world!
/// [info] The weather is nice today.
/// [warn] I almost couldn't, but I did it!
/// ```
#[macro_export]
macro_rules! farve {
    ($func:ident, $prefix:expr, $brace_color:expr) => {
        pub fn $func<S: std::fmt::Display>(msg: S) {
            owo_farver::prettyln!($prefix, msg, $brace_color)
        }
    };
    ($func:ident, $prefix:expr) => {
        pub fn $func<S: std::fmt::Display>(msg: S) {
            owo_farver::prettyln!($prefix, msg)
        }
    };
    ($func:ident) => {
        pub fn $func<S: std::fmt::Display>(msg: S) {
            owo_farver::prettyln!(stringify!($func), msg)
        }
    };
}
/// Just like `farve!`, but prints to stderr instead of stdout.
#[macro_export]
macro_rules! efarve {
    ($func:ident, $prefix:expr, $brace_color:expr) => {
        pub fn $func<S: std::fmt::Display>(msg: S) {
            eprettyln!($prefix, msg, $brace_color)
        }
    };
    ($func:ident, $prefix:expr) => {
        pub fn $func<S: std::fmt::Display>(msg: S) {
            eprettyln!($prefix, msg)
        }
    };
    ($func:ident) => {
        pub fn $func<S: std::fmt::Display>(msg: S) {
            eprettyln!(stringify!($func), msg)
        }
    };
}
