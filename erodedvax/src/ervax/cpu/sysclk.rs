#[derive(Copy, Clone, Debug)]
pub struct SystemClock {
    cycles_per_tick: u32,
    cycles_this_tick: u32,
    cycles_all_time: usize,
}

impl SystemClock {
    pub fn consume_cycles(&mut self, amnt: u32) -> bool {
        self.cycles_this_tick += amnt;
        self.cycles_all_time += amnt as usize;
        return self.cycles_this_tick >= self.cycles_per_tick;
    }

    pub fn new_tick(&mut self) {
        self.cycles_this_tick = 0;
    }
}