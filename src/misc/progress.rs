use std::io::{stdout, Write};

/// Manages a simple ProgressBar that prints to stdout
pub struct ProgressBar {
    buffer: String, // reuse buffer for formatting to avoid allocations
    curr: usize,
    max: usize,
    msg: String,
    last_percent: f32,
}

impl ProgressBar {
    const RUNNER: &'static str = ">";
    const FULL_CHAR: &'static str = "#";
    const EMPTY_CHAR: &'static str = "-";
    const WIDTH: f32 = 50.;

    /// Create a new ProgressBar with the given maximum
    pub fn new(max: usize, msg: String) -> ProgressBar {
        print!(
            "{} [{}{}] 0.00% (0/{})",
            msg,
            Self::RUNNER,
            Self::EMPTY_CHAR.repeat((Self::WIDTH - 1.) as usize),
            max
        );
        ProgressBar {
            buffer: String::with_capacity(80),
            curr: 0,
            max,
            msg,
            last_percent: 0.,
        }
    }

    pub fn reset(&mut self, msg: String) {
        self.msg = msg;
        self.curr = 0;
        self.last_percent = -1.;
        self.next();
    }

    /// Advances the ProgressBar by 1
    /// Only prints, if the difference of percentage exceeds some threshold
    pub fn next(&mut self) {
        self.curr += 1;
        let percent = self.curr as f32 / self.max as f32;
        if self.curr != self.max && percent - self.last_percent <= 0.001 {
            return;
        }

        let full = (Self::WIDTH * percent) as usize;
        let empty = Self::WIDTH as usize - full;
        let runner = if empty > 0 { Self::RUNNER } else { "" };
        let empty = if empty > 0 { empty - 1 } else { 0 };

        self.buffer.clear();

        {
            use std::fmt::Write;
            write!(
                self.buffer,
                "\r{} [{}{}{}] {:.2}% ({}/{}){}",
                self.msg,
                Self::FULL_CHAR.repeat(full),
                runner,
                Self::EMPTY_CHAR.repeat(empty),
                percent * 100.,
                self.curr,
                self.max,
                if self.curr == self.max { '\n' } else { ' ' }
            )
            .unwrap();
        }

        print!("{}", self.buffer);
        stdout().flush().unwrap();
        self.last_percent = percent;
    }
}
