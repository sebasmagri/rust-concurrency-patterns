use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// An expensive way of testing primality
fn is_prime(n: usize) -> bool {
    // this will probably allocate a lot of memory for huge numbers
    (2..n).all(|i| {n % i != 0})
}

fn producer(tx: mpsc::SyncSender<usize>) -> thread::JoinHandle<()> {
    // lets use bigger numbers to be able to perceive the amount of work
    thread::spawn(move || for i in 100_000_000.. {
        tx.send(i).unwrap();
    })
}

fn worker(id: u64, shared_rx: Arc<Mutex<mpsc::Receiver<usize>>>) {
    thread::spawn(move || loop {
        {
            // if we do the calculations in the match arm that gets the actual
            // value from the channel, then we're going to be holding the
            // lock on the receiver and other workers will not be able to
            // receive new values and work.
            let mut n = 0;
            match shared_rx.lock() {
                Ok(rx) => {
                    match rx.try_recv() {
                        Ok(_n) => {
                            n = _n;
                        },
                        Err(_) => ()
                    }
                },
                Err(_) => ()
            }

            if n != 0 {
                if is_prime(n) {
                    println!("worker {} found a prime: {}", id, n);
                }
            }
        }
    });
}

fn main() {
    // Since we are going to send numbers to the channel in an infinite loop,
    // we need to limit the channel's buffer size to avoid leaking memory
    let (tx, rx) = mpsc::sync_channel(1024);
    let shared_rx = Arc::new(Mutex::new(rx));

    for i in 1..5 {
        worker(i, shared_rx.clone());
    }

    producer(tx).join().unwrap();
}
