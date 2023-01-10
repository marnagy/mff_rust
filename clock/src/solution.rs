use std::{
    sync::{
        mpsc::{self, Receiver, Sender, SyncSender},
        Arc, Mutex,
    },
    thread::{sleep, spawn as thread_spawn, Builder, JoinHandle},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[derive(Debug)]
pub struct Clock {
    interval: Duration,
    channels: Arc<Mutex<Vec<Sender<u32>>>>,
}

impl Clock {
    fn get_timestamp() -> u32 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32
    }
    pub fn new(dur: Duration) -> Self {
        let locker = Arc::new(Mutex::new(Vec::new()));
        let clock = Clock {
            interval: dur,
            channels: locker.clone(),
        };
        // start ticking
        let _join_handle = Builder::new()
            .name("Clock thread".to_string())
            .spawn(move || {
                let mut counter = 1_u32;
                //let start_time = Self::get_timestamp();
                loop {
                    sleep(dur);

                    let channels = locker.lock().unwrap();
                    let timestamp = counter;
                    counter += 1;

                    //println!("sending to {} channel", channels.len());
                    for channel in channels.iter() {
                        channel.send(timestamp).unwrap();
                    }
                }
            });

        clock
    }
    pub fn channel(&self) -> CustomChannel {
        let mut channs = self.channels.lock().unwrap();
        let (tx, rx) = mpsc::channel();
        channs.push(tx);
        // custom wrapper to the assignment's specs (no clue why)
        CustomChannel { receiver: rx }
    }
}

#[derive(Debug)]
pub struct CustomChannel {
    receiver: Receiver<u32>,
}

impl CustomChannel {
    pub fn next(&self) -> u32 {
        self.receiver.recv().unwrap()
    }
}

#[derive(Debug)]
pub struct Ticker {
    //<T: FnMut() + Send + 'static> {
    tick_thread_join_handle: JoinHandle<()>,
}

impl Ticker {
    pub fn new<T: FnMut() + Send + 'static>(clock: &Clock, mut func: T) -> Self {
        let channel = clock.channel();
        let ticker = Ticker {
            // start ticking thread
            tick_thread_join_handle: thread_spawn(move || loop {
                channel.next();
                (func)();
            }),
        };

        ticker
    }
}

#[derive(Debug)]
pub struct Alarm {
    tick_thread_join_handle: JoinHandle<()>,
}

impl Alarm {
    pub fn new<T: FnOnce() + Send + 'static>(clock: &Clock, max_ticks: i32, func: T) -> Self {
        let channel = clock.channel();
        
        Alarm {
            // start ticking thread
            tick_thread_join_handle: thread_spawn(move || {
                let mut counter = 0;
                while counter < max_ticks {
                    channel.next();

                    counter += 1;
                }

                (func)();

                // keep channel alive so that the thread does not crash
                loop {
                    channel.next();
                }
            }),
        }
    }
}
