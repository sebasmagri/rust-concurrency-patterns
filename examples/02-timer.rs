use std::sync::mpsc;
use std::thread;
use std::time::Duration;


const NUM_TIMERS: usize = 24;


fn timer(d: usize, tx: mpsc::Sender<usize>) {
    thread::spawn(move || {
        println!("{}: setting timer...", d);
        thread::sleep(Duration::from_secs(d as u64));
        println!("{}: sent!", d);
        tx.send(d).unwrap();
    });
}


fn main() {
    let (tx, rx) = mpsc::channel();
    for i in 0..NUM_TIMERS {
        timer(i, tx.clone());
    }

    for v in rx.iter().take(NUM_TIMERS) {
        println!("{}: received!", v);
    }
}
