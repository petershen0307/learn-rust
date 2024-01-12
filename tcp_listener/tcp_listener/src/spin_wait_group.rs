use std::{
    num::Wrapping,
    sync::{Arc, Mutex},
    thread, time,
};

pub struct WaitGroup {
    state: u64, // high 32 bit for counter, low 32 bit for wait counter
}

impl WaitGroup {
    pub fn new() -> WaitGroup {
        WaitGroup { state: 0 }
    }

    pub fn add(&mut self, delta: i32) {
        self.state = (Wrapping(self.state) + Wrapping((delta as u64) << 32)).0;
        let count = (self.state >> 32) as i32;
        let wait = self.state as i32;
        if count < 0 {
            panic!("sync: negative WaitGroup counter")
        }
        if wait != 0 && delta > 0 && count == (delta as i32) {
            panic!("sync: WaitGroup misuse: Add called concurrently with Wait")
        }
        if count > 0 || wait == 0 {
            return;
        }
        self.state = 0;
    }

    pub fn done(&mut self) {
        self.add(-1);
    }

    fn wait(&mut self) {
        let v = (self.state >> 32) as i32;
        if v == 0 {
            return;
        }
        self.state += 1;
    }

    fn check_state(&self) -> u64 {
        self.state
    }

    pub fn spin_wait(wg: Arc<Mutex<WaitGroup>>) {
        {
            wg.lock().unwrap().wait();
        }
        while { wg.lock().unwrap().check_state() } != 0 {
            thread::sleep(time::Duration::from_millis(50));
        }
    }
}
