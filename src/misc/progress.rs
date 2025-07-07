/// Manages a simple ProgressBar that prints to stdout
pub struct ProgressBar {
    curr: usize,
    max: usize,
    last_percent: f32,
}

impl ProgressBar {
    const RUNNER: &'static str = ">";
    const FULL_CHAR: &'static str = "#";
    const EMPTY_CHAR: &'static str = "-";
    const WIDTH: f32 = 50.;

    /// Create a new ProgressBar with the given maximum
    pub fn new(max: usize) -> ProgressBar {
        print!(
            "[{}{}] 0.00% (0/{})",
            Self::RUNNER,
            Self::EMPTY_CHAR.repeat((Self::WIDTH - 1.) as usize),
            max
        );
        ProgressBar {
            curr: 0,
            max,
            last_percent: 0.,
        }
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
        let full = if full > 0 { full - 1 } else { 0 };

        print!(
            "\r[{}{}{}] {:.2}% ({}/{})",
            Self::FULL_CHAR.repeat(full),
            Self::RUNNER,
            Self::EMPTY_CHAR.repeat(empty),
            percent * 100.,
            self.curr,
            self.max,
        );
        if self.curr == self.max {
            println!();
        }
        self.last_percent = percent;
    }
}
