extern crate owo_colors;

/// Wraps a string on either side with square brackets.
/// The brightness of the brackets can be set with the second argument (0-2).
///
/// `( "s" , 0 | 1 | 2 | _ ) -> "[s]"`
///
/// # Examples
/// ```
/// use owo_colors::OwoColorize;
/// use farve::brackets;
///
/// let i0 = brackets!("INFO");
/// ```
#[macro_export]
macro_rules! brackets {
    ($msg:expr) => {
        format!("{}{}{}", '[', $msg, ']')
    };

    ($level:expr, $bracket_brightness:expr) => {
        match $bracket_brightness {
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

/// Create a new println function with a prefix and brackets (0-2 brightness).
///
/// `level` - any prefix.
/// `msg` - it should not be wrapped in quotes.
/// `bracket_brightness` - the brightness of the brackets (default: 0).
/// # Examples
/// ```
/// use owo_colors::OwoColorize;
/// use farve::prettyln;
///
/// prettyln!("info", "The weather is nice today.");
/// prettyln!("warn".yellow(), "I almost couldn't, but I did it!");
/// ```
#[macro_export]
macro_rules! prettyln {
    ($level:expr, $msg:expr, $bracket_brightness:expr) => {
        println!("{} {}", farve::brackets!($level, $bracket_brightness), $msg)
    };
    ($level:expr, $msg:expr) => {
        println!("{} {}", farve::brackets!($level), $msg)
    };
    ($level:expr) => {
        println!("{} {}", farve::brackets!($level))
    };
}

/// Create a new eprintln function with a prefix and brackets (0-2 brightness).
///
/// The first argument is the name of the function, it should not be wrapped in quotes.
/// The second argument is the prefix, it should be wrapped in quotes (default: function name).
/// The third argument is the brightness of the brackets (default: 0).
#[macro_export]
macro_rules! eprettyln {
    ($level:expr, $msg:expr, $bracket_brightness:expr) => {
        eprintln!("{} {}", farve::brackets!($level, $bracket_brightness), $msg)
    };
    ($level:expr, $msg:expr) => {
        eprintln!("{} {}", farve::brackets!($level), $msg)
    };
    ($level:expr) => {
        eprintln!("{} {}", farve::brackets!($level))
    };
}

/// Create a new println function with a prefix and brackets (0-2 brightness).
///
/// `$func` - the name of the function, it should not be wrapped in quotes.
/// `$prefix` - the prefix, it should be wrapped in quotes (default: function name).
/// `$bracket_brightness` - `0 | 1 | 2`; brightness of the brackets (default: 0).
///
/// # Examples
/// ```
/// use owo_colors::OwoColorize;
/// use farve::farve;
///
/// farve!(silly, "silly 😋".white().bold(), 2);
/// farve!(info);
/// farve!(warn, "warn".yellow().underline());
///
///
/// silly("Hello, world!");
/// info("The weather is nice today.");
/// warn("I almost couldn't, but I did it!");
/// ```
/// # Output (imagine color)
/// ```log
/// [silly 😋] Hello, world!
/// [info] The weather is nice today.
/// [warn] I almost couldn't, but I did it!
/// ```
#[macro_export]
macro_rules! farve {
    ($func:ident, $prefix:expr, $bracket_brightness:expr) => {
        pub fn $func<S: std::fmt::Display>(msg: S) {
            farve::prettyln!($prefix, msg, $bracket_brightness)
        }
    };
    ($func:ident, $prefix:expr) => {
        pub fn $func<S: std::fmt::Display>(msg: S) {
            farve::prettyln!($prefix, msg)
        }
    };
    ($func:ident) => {
        pub fn $func<S: std::fmt::Display>(msg: S) {
            farve::prettyln!(stringify!($func), msg)
        }
    };
}
/// Just like `farve!`, but prints to stderr instead of stdout. Creates an eprintln with a prefix and brackets (0-2 brightness).
///
/// `$func` - the name of the function, it should not be wrapped in quotes.
/// `$prefix` - the prefix, it should be wrapped in quotes (default: function name).
/// `$bracket_brightness` - `0 | 1 | 2`; brightness of the brackets (default: 0).
///
/// # Examples
/// ```
/// use owo_colors::OwoColorize;
/// use farve::efarve;
///
/// efarve!(warn, "WARN".yellow().underline(), 1);
/// efarve!(skip, "skip".bright_black().italic());
#[macro_export]
macro_rules! efarve {
    ($func:ident, $prefix:expr, $bracket_brightness:expr) => {
        pub fn $func<S: std::fmt::Display>(msg: S) {
            farve::eprettyln!($prefix, msg, $bracket_brightness)
        }
    };
    ($func:ident, $prefix:expr) => {
        pub fn $func<S: std::fmt::Display>(msg: S) {
            farve::eprettyln!($prefix, msg)
        }
    };
    ($func:ident) => {
        pub fn $func<S: std::fmt::Display>(msg: S) {
            farve::eprettyln!(stringify!($func), msg)
        }
    };
}
