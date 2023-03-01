use std::sync::atomic::{AtomicUsize, Ordering};
const DEFAULT_NAME: String = String::new();
static ALLOCATED_NAMES: AtomicUsize = AtomicUsize::new(0);

pub struct Robot{
    name: String,
}
impl Default for Robot{
    fn default() -> Self {
        Self::new()
    }
}

impl Robot {
    pub fn new() -> Self {
        //unimplemented!("Construct a new Robot struct.");
        let mut ron = Robot { name: DEFAULT_NAME };
        ron.reset_name();
        ron
    }

    pub fn name(&self) -> &str {
        //unimplemented!("Return the reference to the robot's name.");
        &self.name
    }

    pub fn reset_name(&mut self) {
        //unimplemented!("Assign a new unique name to the robot.");
        let mut seq = ALLOCATED_NAMES.fetch_add(1, Ordering::SeqCst);
        let mut roid=String::new();
        for _ in 0..2{
            roid.push((b'A'+(seq % 26)as u8)as char);
            seq /=26;
        }
        for _ in 0..3{
            roid.push((b'0'+(seq % 10)as u8)as char);
            seq /=10;
        }
        self.name = roid;
    }
}
