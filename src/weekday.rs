use chrono;

pub struct Weekday {
    pub weekday: chrono::Weekday
}

impl Weekday {
    pub fn new() -> Weekday {
        Weekday { weekday: chrono::Weekday::Sun }
    }
}

impl Iterator for Weekday {
    type Item = chrono::Weekday;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.weekday.succ();
        self.weekday = next;
        Some(next)
    }
}
