//! TTY format

use self::Format::{Blue, Bold, Cyan, Green, Magenta, Red, Underline, Yellow};

/// Format patterns
enum Format {
    Blue,
    Bold,
    Cyan,
    Green,
    Magenta,
    Red,
    Underline,
    Yellow,
}

impl Format {
    /// Number of format style
    pub fn to_u16(&self) -> u16 {
        match *self {
            Blue => 34,
            Bold => 1,
            Cyan => 36,
            Green => 32,
            Magenta => 35,
            Red => 31,
            Underline => 4,
            Yellow => 33,
        }
    }
}

/// Returns TTY formatted strings
fn make_color(format: Format, s: &str) -> String {
    let code = format.to_u16();
    format!("\x1b[{}m{}\x1b[0m", code, s)
}

/// Returns formatted strings with blue
pub fn blue(s: &str) -> String {
    make_color(Format::Blue, s)
}

/// Returns formatted strings with bold
pub fn bold(s: &str) -> String {
    make_color(Format::Bold, s)
}

/// Returns formatted strings with cyan
pub fn cyan(s: &str) -> String {
    make_color(Format::Cyan, s)
}

/// Returns formatted strings with green
pub fn green(s: &str) -> String {
    make_color(Format::Green, s)
}

/// Returns formatted strings with magenta
pub fn magenta(s: &str) -> String {
    make_color(Format::Magenta, s)
}

/// Returns formatted strings with red
pub fn red(s: &str) -> String {
    make_color(Format::Red, s)
}

/// Returns formatted strings with underline
pub fn underline(s: &str) -> String {
    make_color(Format::Underline, s)
}

/// Returns formatted strings with yellow
pub fn yellow(s: &str) -> String {
    make_color(Format::Yellow, s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_blue() {
        let expected = "\x1b[34mblue\x1b[0m";
        assert_eq!(expected, blue("blue"));
    }

    #[test]
    fn format_bold() {
        let expected = "\x1b[1mbold\x1b[0m";
        assert_eq!(expected, bold("bold"));
    }

    #[test]
    fn format_cyan() {
        let expected = "\x1b[36mcyan\x1b[0m";
        assert_eq!(expected, cyan("cyan"));
    }

    #[test]
    fn format_green() {
        let expected = "\x1b[32mgreen\x1b[0m";
        assert_eq!(expected, green("green"));
    }

    #[test]
    fn format_magenta() {
        let expected = "\x1b[35mmagenta\x1b[0m";
        assert_eq!(expected, magenta("magenta"));
    }

    #[test]
    fn format_red() {
        let expected = "\x1b[31mred\x1b[0m";
        assert_eq!(expected, red("red"));
    }

    #[test]
    fn format_underline() {
        let expected = "\x1b[4munderline\x1b[0m";
        assert_eq!(expected, underline("underline"));
    }

    #[test]
    fn format_yellow() {
        let expected = "\x1b[33myellow\x1b[0m";
        assert_eq!(expected, yellow("yellow"));
    }
}
