use std::thread;
use std::time::Duration;

struct HasDrop{
    pub name: u32
}

impl Drop for HasDrop {
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
    }
}

fn main() {
    {
        let _x = HasDrop{name: 1};
    }
    let _y = HasDrop{name: 2};
    thread::sleep(Duration::from_millis(5000));
}
