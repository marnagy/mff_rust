mod solution;

use core::time;
use solution::{Clock, Ticker /* Alarm */};
use std::{thread, thread::Thread, time::Duration};

fn main() {
    let mut clock = Clock::new(Duration::from_secs(1));
    let mut clock2 = Clock::new(Duration::from_secs(2));
    //let ticker = Ticker::new(&clock, || println!("Hi from ticker!"));
    //let alarm = Alarm::new(&clock, 1, || println!("Hi from alarm"));

    //thread::sleep(Duration::from_secs(5));
    let channel = clock.channel();
    let channel2 = clock2.channel();
    for i in 0..5 {
        println!("Receiveing {}-th massage", i);
        let timestamp = channel.next();
        let timestamp2 = channel2.next();
        println!("Received timestamp: {}", timestamp);
        println!("Received timestamp2: {}", timestamp2);
    }

    thread::sleep(Duration::from_secs(5));
}
