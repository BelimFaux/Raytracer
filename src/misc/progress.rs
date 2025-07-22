use std::fmt::Write;

/// Manages a simple Progressbar that prints to stdout
pub struct ProgressBar {
    buffer: String, // reuse buffer for formatting to avoid allocations
    curr: usize,
    max: usize,
    msg: String,
    last_percent: f64,
}

impl ProgressBar {
    const RUNNER: &'static str = ">";
    const FULL_CHAR: &'static str = "#";
    const EMPTY_CHAR: &'static str = "-";
    const WIDTH: f64 = 50.;

    /// Create a new ``ProgressBar`` with the given maximum
    #[must_use]
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
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

    /// Advances the progress bar by 1
    /// Only prints, if the difference of percentage exceeds some threshold
    pub fn next(&mut self) {
        self.curr += 1;
        #[allow(clippy::cast_precision_loss)]
        let percent = self.curr as f64 / self.max as f64;
        if self.curr != self.max && percent - self.last_percent <= 0.001 {
            return;
        }

        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let full = (Self::WIDTH * percent) as usize;
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let empty = Self::WIDTH as usize - full;
        let runner = if empty > 0 { Self::RUNNER } else { "" };
        let empty = if empty > 0 { empty - 1 } else { 0 };

        self.buffer.clear();

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

        print!("{}", self.buffer);
        self.last_percent = percent;
    }
}
