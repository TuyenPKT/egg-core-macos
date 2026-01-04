use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

static RUNNING: AtomicBool = AtomicBool::new(false);

pub fn start(threads: u32) {
    RUNNING.store(true, Ordering::SeqCst);

    for _ in 0..threads {
        thread::spawn(|| {
            while RUNNING.load(Ordering::SeqCst) {
                // PoW loop thật (sau này gắn)
            }
        });
    }
}

pub fn stop() {
    RUNNING.store(false, Ordering::SeqCst);
}
