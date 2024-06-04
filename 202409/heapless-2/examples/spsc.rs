use heapless::spsc;
use std::thread;

fn main() {
    let q: &'static mut spsc::Queue::<u8, 8> = {
        static mut Q: spsc::Queue::<u8, 8> = spsc::Queue::<u8, 8>::new();
        unsafe { &mut Q }
    };
    let (mut producer, mut consumer) = q.split();

    // コンパイルエラー: producer / consumer はコピーできない
    // let another_producer = producer;
    // let another_consumer = consumer;
    // コンパイルエラー: split後のqは使えない
    // q.enqueue(0u8).ok();

    thread::spawn(move || {
        let mut sent = 0;
        loop {
            producer.enqueue(sent).ok();
            sent += 1;
            thread::sleep(std::time::Duration::from_millis(1_000));
        }
    });

    loop {
        let _ = consumer.dequeue()
            .map(|b| {
                println!("received: {:?}", b);
            });
        thread::sleep(std::time::Duration::from_millis(1000));
    }
}
