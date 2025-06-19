pub struct ProgressBar {
    curr: usize,
    max: usize,
    last_percent: f32,
}

impl ProgressBar {
    const FULL_CHAR: &str = "#";
    const EMPTY_CHAR: &str = "-";

    pub fn new(max: usize) -> ProgressBar {
        ProgressBar {
            curr: 0,
            max,
            last_percent: 0.,
        }
    }

    pub fn next(&mut self) {
        self.curr += 1;
        const WIDTH: f32 = 50.;
        let percent = self.curr as f32 / self.max as f32;
        if self.curr != self.max && percent - self.last_percent <= 0.001 {
            return;
        }

        let full = (WIDTH * percent) as usize;
        let empty = WIDTH as usize - full;
        print!(
            "\r[{}{}] {:.2}% ({}/{})",
            Self::FULL_CHAR.repeat(full),
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
