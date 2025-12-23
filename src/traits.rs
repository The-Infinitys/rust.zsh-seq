pub trait ColoredTermPrompt {
    fn bold(self) -> String;
    fn underline(self) -> String;
    fn red(self) -> String;
    fn green(self) -> String;
    fn yellow(self) -> String;
    fn blue(self) -> String;
    fn white(self) -> String;
    fn on_yellow(self) -> String;
    fn on_blue(self) -> String;
    fn rgb_color(self, r: u8, g: u8, b: u8) -> String;
    fn on_rgb_color(self, r: u8, g: u8, b: u8) -> String;
}

impl<T: AsRef<str>> ColoredTermPrompt for T {
    fn bold(self) -> String {
        format!("%B{}%b", self.as_ref())
    }

    fn underline(self) -> String {
        format!("%U{}%u", self.as_ref())
    }

    fn red(self) -> String {
        format!("%F{{red}}{}%f", self.as_ref())
    }

    fn green(self) -> String {
        format!("%F{{green}}{}%f", self.as_ref())
    }

    fn yellow(self) -> String {
        format!("%F{{yellow}}{}%f", self.as_ref())
    }

    fn blue(self) -> String {
        format!("%F{{blue}}{}%f", self.as_ref())
    }

    fn white(self) -> String {
        format!("%F{{white}}{}%f", self.as_ref())
    }

    fn on_yellow(self) -> String {
        format!("%K{{yellow}}{}%k", self.as_ref())
    }

    fn on_blue(self) -> String {
        format!("%K{{blue}}{}%k", self.as_ref())
    }

    fn rgb_color(self, r: u8, g: u8, b: u8) -> String {
        format!(
            "%{{\x1b[38;2;{};{};{}m%}}{}%{{\x1b[0m%}}",
            r,
            g,
            b,
            self.as_ref()
        )
    }

    fn on_rgb_color(self, r: u8, g: u8, b: u8) -> String {
        format!(
            "%{{\x1b[48;2;{};{};{}m%}}{}%{{\x1b[0m%}}",
            r,
            g,
            b,
            self.as_ref()
        )
    }
}
