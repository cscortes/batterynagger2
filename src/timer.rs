use std::sync::mpsc::{Receiver, channel};
use std::thread;
use std::time::Duration;

pub struct TimerObject {
    rx : Receiver<u8>
}

impl TimerObject {
    pub fn new(seconds: u64, times: Option<u64>) -> TimerObject {
        TimerObject {
            rx : TimerObject::build_timer(seconds, times),
        }
    }

    pub fn triggered (&self) -> bool {
        self.rx.try_recv().is_ok()
    }

    // if None, it becomes an infinite timer
    // otherwise times controls number of times it will call triggered
    // a value of Some(1) will effectively making it a oneshot
    //
    fn build_timer(seconds: u64, times: Option<u64> ) -> Receiver<u8> {
        // Create channels for sending and receiving
        let (one_tx, one_rx) = channel();

        // Spawn one second timer
        thread::spawn(move || {
            let mut times = times;
            loop {

                match times {
                    Some(n) if n > 0 => times = Some(n - 1),
                    Some(_)  => break,  // n = 0 or less than 0
                    None => (),         // no tracking needed, just keep going
                }

                thread::sleep(Duration::from_secs(seconds));

                // Had to add this.  Although it seems like 
                // you may not need this, actually you do
                // in the off chance that the receiver is dead.  
                if one_tx.send(0).is_err() {
                    //println!("Receiver is dead");
                }
            }
        }); 

        one_rx
    }

}


