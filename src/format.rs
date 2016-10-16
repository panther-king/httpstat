//! TTY format
use self::Format::{Blue, Bold, Cyan, GrayScale, Green, Magenta, Red, Underline, Yellow};

/// Format patterns
pub enum Format {
    Blue,
    Bold,
    Cyan,
    GrayScale(u16),
    Green,
    Magenta,
    Red,
    Underline,
    Yellow,
}

impl Format {
    /// Returns formatted strings for TTY
    pub fn tty(&self, s: &str) -> String {
        let code = match *self {
            GrayScale(i) => format!("38;5;{}", i + 232),
            _ => format!("{}", self.to_number()),
        };

        self.make_color(&code, s)
    }

    /// Strings of TTY format
    fn make_color(&self, code: &str, s: &str) -> String {
        format!("\x1b[{}m{}\x1b[0m", code, s)
    }

    /// Number of format style
    fn to_number(&self) -> u16 {
        match *self {
            Blue => 34,
            Bold => 1,
            Cyan => 36,
            GrayScale(i) => i,
            Green => 32,
            Magenta => 35,
            Red => 31,
            Underline => 4,
            Yellow => 33,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Format::*;

    #[test]
    fn format_blue() {
        let expected = "\x1b[34mblue\x1b[0m";
        assert_eq!(expected, Blue.tty("blue"));
    }

    #[test]
    fn format_bold() {
        let expected = "\x1b[1mbold\x1b[0m";
        assert_eq!(expected, Bold.tty("bold"));
    }

    #[test]
    fn format_cyan() {
        let expected = "\x1b[36mcyan\x1b[0m";
        assert_eq!(expected, Cyan.tty("cyan"));
    }

    #[test]
    fn format_gray_scale() {
        let expected = "\x1b[38;5;248mgrayscale\x1b[0m";
        assert_eq!(expected, GrayScale(16).tty("grayscale"));
    }

    #[test]
    fn format_green() {
        let expected = "\x1b[32mgreen\x1b[0m";
        assert_eq!(expected, Green.tty("green"));
    }

    #[test]
    fn format_magenta() {
        let expected = "\x1b[35mmagenta\x1b[0m";
        assert_eq!(expected, Magenta.tty("magenta"));
    }

    #[test]
    fn format_red() {
        let expected = "\x1b[31mred\x1b[0m";
        assert_eq!(expected, Red.tty("red"));
    }

    #[test]
    fn format_underline() {
        let expected = "\x1b[4munderline\x1b[0m";
        assert_eq!(expected, Underline.tty("underline"));
    }

    #[test]
    fn format_yellow() {
        let expected = "\x1b[33myellow\x1b[0m";
        assert_eq!(expected, Yellow.tty("yellow"));
    }
}
