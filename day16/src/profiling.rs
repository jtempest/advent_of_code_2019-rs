use std::time::Instant;

pub struct Timer<'a> {
    start_time: Instant,
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        Timer {
            start_time: Instant::now(),
            name,
        }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        let duration = Instant::now() - self.start_time;
        println!("{}: {:?}", self.name, duration);
    }
}
