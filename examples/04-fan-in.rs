use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn producer(id: u64, tx: mpsc::Sender<usize>) {
    thread::spawn(move || {
        let mut i = 0;
        loop {
            i += 1;
            println!("producer {} sending: {}", id, i);
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(350 * id));
        }
    });
}

fn consumer(rx: Arc<Mutex<mpsc::Receiver<usize>>>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        for v in rx.lock().unwrap().iter() {
            println!("consumer got: {}", v);
        }
    })
}

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 1..5 {
        producer(i, tx.clone());
    }

    consumer(Arc::new(Mutex::new(rx))).join();
}
