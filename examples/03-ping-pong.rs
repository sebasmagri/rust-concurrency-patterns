use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;


const START_DELAY: u64 = 3;
const GAME_DURATION: u64 = 1;
// if it's higher than 2, sequential order is not guarranteed
const NUM_PLAYERS: usize = 2;
const PING_PONG_FREQ_MS: u64 = 100;


fn player(id: usize,
          racket: mpsc::Sender<Mutex<usize>>,
          table: Arc<Mutex<mpsc::Receiver<Mutex<usize>>>>)
          -> thread::JoinHandle<()> {

    thread::spawn(move || {
        // Player is alive in its loop
        println!("Player {}: ready...", id);
        loop {

            // scope of the rx lock
            {
                let locked_table = table.lock().unwrap();
                let ball = (*locked_table).recv().unwrap();

                // scope of the ball lock
                {
                    let mut hit_count = ball.lock().unwrap();
                    println!("Player {}: PING (hc = {})", id, *hit_count);

                    // increase the ball's hit count
                    *hit_count += 1;
                    println!("Player {}: PONG (hc = {})", id, *hit_count);
                } // unlock the ball here to send it to the other player(s)

                // send the ball to the other player(s)
                racket.send(ball).unwrap();
            } // unlock the rx here for other player(s) to receive the ball

            // wait for the ball to come back
            thread::sleep(Duration::from_millis(PING_PONG_FREQ_MS));
        }
    })
}


fn main() {
    let (racket, table) = mpsc::channel();
    let shared_table = Arc::new(Mutex::new(table));
    let mut players: Vec<thread::JoinHandle<()>> = Vec::new();

    for id in 0..NUM_PLAYERS {
        players.push(player(id, racket.clone(), shared_table.clone()));
    }

    println!("The game is starting in {} seconds...", START_DELAY);
    thread::sleep(Duration::from_secs(START_DELAY));

    println!("Game!");
    racket.send(Mutex::new(0)).unwrap();

    thread::sleep(Duration::from_secs(GAME_DURATION));

    println!("{} seconds elapsed, game finished...", GAME_DURATION);
    // lock the rx
    let referee_table = shared_table.clone();
    let table_lock = referee_table.lock().unwrap();

    // get the ball
    table_lock.recv().unwrap();
}
