use heapless::mpmc;
use std::thread;

fn main() {
   static Q: mpmc::MpMcQueue<u8, 4> = mpmc::MpMcQueue::new();

    thread::spawn(|| {
        let mut sent = 0;
        loop {
            Q.enqueue(sent).ok();
            sent += 1;
            thread::sleep(std::time::Duration::from_millis(1_000));
        }
    });

    loop {
        let _ = Q.dequeue()
            .map(|b| {
                println!("received: {:?}", b);
            });
        thread::sleep(std::time::Duration::from_millis(1000));
    }
}
